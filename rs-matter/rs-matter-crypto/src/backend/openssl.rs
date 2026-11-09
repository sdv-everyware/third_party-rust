/*
 *
 *    Copyright (c) 2026 Project CHIP Authors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

//! An OpenSSL-based crypto backend
//!
//! The backend is only propagating errors originating from incorrect input data, such as invalid keys.
//! All other errors (including out of memory - for now) are considered unrecoverable and will cause a panic.
//! This is done so that programming errors are caught early-on, with meaningful location and backtraces,
//! even in `no_std` environments where backtrace capturing during error propagation does not work.

#![allow(deprecated)] // Remove this once `hmac` updates to `generic-array` 1.x

use core::ops::Mul;

use openssl::asn1::Asn1Type;
use openssl::bn::{BigNum, BigNumContext};
use openssl::cipher::CipherRef;
use openssl::cipher_ctx::CipherCtx;
use openssl::derive::Deriver;
use openssl::ec::{EcGroup, EcKey, EcPoint, PointConversionForm};
use openssl::ecdsa::EcdsaSig;
use openssl::hash::{Hasher, MessageDigest};
use openssl::md::{Md, MdRef};
use openssl::nid::Nid;
use openssl::pkey::{Id, PKey, Private, Public};
use openssl::pkey_ctx::PkeyCtx;
use openssl::x509::{X509NameBuilder, X509ReqBuilder};

// We directly use the hmac crate here, there was a self-referential structure
// problem while using OpenSSL's Signer
// TODO: Use proper OpenSSL method for this
use hmac::{Hmac, KeyInit, Mac};

use rand_core::{TryCryptoRng, TryRng};

use crate::error::{Error, ErrorCode};
use crate::{CanonPkcSecretKeyRef, CanonUint320Ref, CryptoSensitive, CryptoSensitiveRef};

macro_rules! openssl_check {
    ($expr:expr) => {{
        let result = $expr;
        match result {
            Ok(val) => Ok::<_, Error>(val),
            Err(err) => {
                #[cfg(not(feature = "defmt"))]
                {
                    panic!("{:?}", err);
                }

                #[cfg(feature = "defmt")]
                {
                    extern crate alloc;
                    panic!("{}", alloc::format!("{err:?}"));
                }
            }
        }
    }};
}

/// An OpenSSL-based crypto backend
pub struct OpenSslCrypto<'s> {
    /// Elliptic curve group (secp256r1)
    ec_group: ECGroup<{ crate::EC_CANON_POINT_LEN }, { crate::EC_CANON_SCALAR_LEN }>,
    /// The singleton secret key to be returned by `Crypto::singleton_singing_secret_key`
    singleton_secret_key: CanonPkcSecretKeyRef<'s>,
}

impl<'s> OpenSslCrypto<'s> {
    /// Create a new OpenSSL crypto backend
    ///
    /// # Arguments
    /// - `singleton_secret_key` - A singleton secret key to be returned by `Crypto::singleton_singing_secret_key`
    ///   The primary use-case for this secret key is to be used as the secret key for the Device Attestation credentials
    pub fn new(singleton_secret_key: CanonPkcSecretKeyRef<'s>) -> Self {
        Self {
            ec_group: unsafe { ECGroup::new(Nid::X9_62_PRIME256V1).unwrap() },
            singleton_secret_key,
        }
    }
}

impl crate::Crypto for OpenSslCrypto<'_> {
    type Rand<'a>
        = Rand
    where
        Self: 'a;

    type WeakRand<'a>
        = Rand
    where
        Self: 'a;

    type Hash<'a>
        = Hash<{ crate::HASH_LEN }>
    where
        Self: 'a;

    type Hash1<'a>
        = Hash1<{ crate::SHA1_HASH_LEN }>
    where
        Self: 'a;

    type Hmac<'a>
        = HmacSha256
    where
        Self: 'a;

    type Kdf<'a>
        = Hkdf
    where
        Self: 'a;

    type PbKdf<'a>
        = Pbkdf2Hmac
    where
        Self: 'a;

    type Aead<'a>
        = Aead<{ crate::AEAD_CANON_KEY_LEN }, { crate::AEAD_NONCE_LEN }, { crate::AEAD_TAG_LEN }>
    where
        Self: 'a;

    type PublicKey<'a>
        = ECPoint<'a, { crate::EC_CANON_POINT_LEN }, { crate::EC_CANON_SCALAR_LEN }>
    where
        Self: 'a;

    type SecretKey<'a>
        = ECScalar<'a, { crate::EC_CANON_SCALAR_LEN }, { crate::EC_CANON_POINT_LEN }>
    where
        Self: 'a;

    type SigningSecretKey<'a>
        = ECScalar<'a, { crate::EC_CANON_SCALAR_LEN }, { crate::EC_CANON_POINT_LEN }>
    where
        Self: 'a;

    type EcScalar<'a>
        = ECScalar<'a, { crate::EC_CANON_SCALAR_LEN }, { crate::EC_CANON_POINT_LEN }>
    where
        Self: 'a;

    type EcPoint<'a>
        = ECPoint<'a, { crate::EC_CANON_POINT_LEN }, { crate::EC_CANON_SCALAR_LEN }>
    where
        Self: 'a;

    fn rand(&self) -> Result<Self::Rand<'_>, Error> {
        Ok(Rand(()))
    }

    fn weak_rand(&self) -> Result<Self::WeakRand<'_>, Error> {
        Ok(Rand(()))
    }

    fn hash(&self) -> Result<Self::Hash<'_>, Error> {
        unsafe { Hash::new(MessageDigest::sha256()) }
    }

    fn hash1(&self) -> Result<Self::Hash1<'_>, Error> {
        unsafe { Hash1::new(MessageDigest::sha1()) }
    }

    fn hmac<const KEY_LEN: usize>(
        &self,
        key: CryptoSensitiveRef<'_, KEY_LEN>,
    ) -> Result<Self::Hmac<'_>, Error> {
        HmacSha256::new(key.access())
    }

    fn kdf(&self) -> Result<Self::Kdf<'_>, Error> {
        Ok(Hkdf::new(Md::sha256()))
    }

    fn pbkdf(&self) -> Result<Self::PbKdf<'_>, Error> {
        Ok(Pbkdf2Hmac::new(MessageDigest::sha256()))
    }

    fn aead(&self) -> Result<Self::Aead<'_>, Error> {
        // BoringSSL has no EVP_CIPHER for AES-CCM, only the AEAD with Matter's parameters.
        Ok(unsafe { Aead::new(&*bssl_aead::EVP_aead_aes_128_ccm_matter()) })
    }

    fn pub_key(&self, key: crate::CanonPkcPublicKeyRef<'_>) -> Result<Self::PublicKey<'_>, Error> {
        self.ec_point(key)
    }

    fn generate_secret_key(&self) -> Result<Self::SecretKey<'_>, Error> {
        self.generate_ec_scalar() // TODO: Should be non-zero
    }

    fn secret_key(
        &self,
        key: crate::CanonPkcSecretKeyRef<'_>,
    ) -> Result<Self::SecretKey<'_>, Error> {
        self.ec_scalar(key)
    }

    fn singleton_singing_secret_key(&self) -> Result<Self::SigningSecretKey<'_>, Error> {
        self.ec_scalar(self.singleton_secret_key)
    }

    fn ec_scalar(&self, scalar: crate::CanonEcScalarRef<'_>) -> Result<Self::EcScalar<'_>, Error> {
        let scalar = openssl_check!(BigNum::from_slice(scalar.access()))?;

        Ok(Self::EcScalar {
            group: &self.ec_group,
            scalar,
        })
    }

    fn ec_scalar_mod_p(&self, uint: CanonUint320Ref<'_>) -> Result<Self::EcScalar<'_>, Error> {
        let mut ctx = openssl_check!(BigNumContext::new())?;
        let mut order = openssl_check!(BigNum::new())?;

        openssl_check!(self.ec_group.group.order(&mut order, &mut ctx))?;

        let uint = openssl_check!(BigNum::from_slice(uint.access()))?;

        let mut scalar = openssl_check!(BigNum::new())?;

        openssl_check!(scalar.checked_rem(&uint, &order, &mut ctx))?;

        Ok(Self::EcScalar {
            group: &self.ec_group,
            scalar,
        })
    }

    fn generate_ec_scalar(&self) -> Result<Self::EcScalar<'_>, Error> {
        let mut ctx = openssl_check!(BigNumContext::new())?;
        let mut order = openssl_check!(BigNum::new())?;
        openssl_check!(self.ec_group.group.order(&mut order, &mut ctx))?;

        let mut scalar = openssl_check!(BigNum::new())?;
        openssl_check!(order.rand_range(&mut scalar))?;

        Ok(Self::EcScalar {
            group: &self.ec_group,
            scalar,
        })
    }

    fn ec_point(&self, point: crate::CanonEcPointRef<'_>) -> Result<Self::EcPoint<'_>, Error> {
        let mut ctx = openssl_check!(BigNumContext::new())?;

        let point = EcPoint::from_bytes(&self.ec_group.group, point.access(), &mut ctx)
            .map_err(|e| map_err("Failed to parse EC point", e))?;

        Ok(Self::EcPoint {
            group: &self.ec_group,
            point,
        })
    }

    fn ec_generator_point(&self) -> Result<Self::EcPoint<'_>, Error> {
        let point = openssl_check!(self
            .ec_group
            .group
            .generator()
            .to_owned(&self.ec_group.group))?;

        Ok(Self::EcPoint {
            group: &self.ec_group,
            point,
        })
    }
}

/// A cryptographically secure random number generator using OpenSSL
#[derive(Copy, Clone)]
pub struct Rand(());

impl TryRng for Rand {
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        rand_core::utils::next_word_via_fill(self)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        rand_core::utils::next_word_via_fill(self)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        openssl::rand::rand_bytes(dest).unwrap();

        Ok(())
    }
}

impl TryCryptoRng for Rand {}

/// A hash implementation
#[derive(Clone)]
pub struct Hash<const HASH_LEN: usize>(Hasher);

impl<const HASH_LEN: usize> Hash<HASH_LEN> {
    /// Create a new hash instance
    ///
    /// # Arguments
    /// - `md`: The message digest algorithm to use
    ///
    /// # Safety
    /// This function is unsafe because the caller must ensure that the
    /// `md` parameter corresponds to the `HASH_LEN` generic parameter.
    unsafe fn new(md: MessageDigest) -> Result<Self, Error> {
        Ok(Self(openssl_check!(Hasher::new(md))?))
    }
}

impl<const HASH_LEN: usize> crate::Digest<HASH_LEN> for Hash<HASH_LEN> {
    fn update(&mut self, data: &[u8]) -> Result<(), Error> {
        openssl_check!(self.0.update(data))
    }

    fn finish_current(&mut self, hash: &mut CryptoSensitive<HASH_LEN>) -> Result<(), Error> {
        let copy = self.clone();

        copy.finish(hash)
    }

    fn finish(mut self, hash: &mut CryptoSensitive<HASH_LEN>) -> Result<(), Error> {
        let digest = openssl_check!(self.0.finish())?;
        hash.access_mut().copy_from_slice(digest.as_ref());

        Ok(())
    }
}

/// A hash implementation
#[derive(Clone)]
pub struct Hash1<const SHA1_HASH_LEN: usize>(Hasher);

impl<const SHA1_HASH_LEN: usize> Hash1<SHA1_HASH_LEN> {
    /// Create a new hash instance
    ///
    /// # Arguments
    /// - `md`: The message digest algorithm to use
    ///
    /// # Safety
    /// This function is unsafe because the caller must ensure that the
    /// `md` parameter corresponds to the `HASH_LEN` generic parameter.
    unsafe fn new(md: MessageDigest) -> Result<Self, Error> {
        Ok(Self(openssl_check!(Hasher::new(md))?))
    }
}

impl<const SHA1_HASH_LEN: usize> crate::Digest<SHA1_HASH_LEN> for Hash1<SHA1_HASH_LEN> {
    fn update(&mut self, data: &[u8]) -> Result<(), Error> {
        openssl_check!(self.0.update(data))
    }

    fn finish_current(&mut self, hash: &mut CryptoSensitive<SHA1_HASH_LEN>) -> Result<(), Error> {
        let copy = self.clone();

        copy.finish(hash)
    }

    fn finish(mut self, hash: &mut CryptoSensitive<SHA1_HASH_LEN>) -> Result<(), Error> {
        let digest = openssl_check!(self.0.finish())?;
        hash.access_mut().copy_from_slice(digest.as_ref());

        Ok(())
    }
}

/// An HMAC-SHA256 implementation
#[derive(Clone)]
pub struct HmacSha256(Hmac<sha2::Sha256>);

impl HmacSha256 {
    /// Create a new HMAC-SHA256 instance
    fn new(key: &[u8]) -> Result<Self, Error> {
        Hmac::<sha2::Sha256>::new_from_slice(key)
            .map(Self)
            .map_err(|_| {
                error!("HMAC initialization failed - invalid key length");

                ErrorCode::InvalidData.into()
            })
    }
}

impl crate::Digest<{ crate::HASH_LEN }> for HmacSha256 {
    fn update(&mut self, data: &[u8]) -> Result<(), Error> {
        Mac::update(&mut self.0, data);

        Ok(())
    }

    fn finish_current(
        &mut self,
        hash: &mut CryptoSensitive<{ crate::HASH_LEN }>,
    ) -> Result<(), Error> {
        let copy = self.clone();

        copy.finish(hash)
    }

    fn finish(self, hash: &mut CryptoSensitive<{ crate::HASH_LEN }>) -> Result<(), Error> {
        hash.access_mut()
            .copy_from_slice(self.0.finalize().into_bytes().as_slice());

        Ok(())
    }
}

/// An HKDF implementation
pub struct Hkdf(&'static MdRef);

impl Hkdf {
    /// Create a new HKDF instance
    fn new(md: &'static MdRef) -> Self {
        Self(md)
    }
}

impl crate::Kdf for Hkdf {
    fn expand<const IKM_LEN: usize, const KEY_LEN: usize>(
        self,
        salt: &[u8],
        ikm: CryptoSensitiveRef<'_, IKM_LEN>,
        info: &[u8],
        key: &mut CryptoSensitive<KEY_LEN>,
    ) -> Result<(), Error> {
        let mut ctx = openssl_check!(PkeyCtx::new_id(Id::HKDF))?;

        openssl_check!(ctx.derive_init())?;

        openssl_check!(ctx.set_hkdf_md(self.0))?;
        openssl_check!(ctx.set_hkdf_key(ikm.access()))?;

        if !salt.is_empty() {
            openssl_check!(ctx.set_hkdf_salt(salt))?;
        }

        openssl_check!(ctx.add_hkdf_info(info))?;

        let len = ctx
            .derive(Some(key.access_mut()))
            .map_err(|e| map_err("HKDF expansion failed", e))?;

        assert_eq!(len, KEY_LEN);

        Ok(())
    }
}

/// A PBKDF2-HMAC implementation
pub struct Pbkdf2Hmac(MessageDigest);

impl Pbkdf2Hmac {
    /// Create a new PBKDF2-HMAC instance
    fn new(md: MessageDigest) -> Self {
        Self(md)
    }
}

impl crate::PbKdf for Pbkdf2Hmac {
    fn derive<const PASS_LEN: usize, const KEY_LEN: usize>(
        self,
        pass: CryptoSensitiveRef<'_, PASS_LEN>,
        iter: usize,
        salt: &[u8],
        key: &mut CryptoSensitive<KEY_LEN>,
    ) -> Result<(), Error> {
        openssl::pkcs5::pbkdf2_hmac(pass.access(), salt, iter, self.0, key.access_mut())
            .map_err(|e| map_err("PBKDF2-HMAC derivation failed", e))
    }
}

/// An AEAD implementation
pub struct Aead<const KEY_LEN: usize, const NONCE_LEN: usize, const TAG_LEN: usize>(
    &'static bssl_aead::EVP_AEAD,
);

/// BoringSSL's AEAD API (openssl/aead.h). BoringSSL, the OpenSSL of Android, has AES-CCM only
/// as an AEAD: EVP_aead_aes_128_ccm_matter (16-byte tag, L=2, the 13-byte nonce of Matter).
/// Declared here: bssl_sys, the crate with BoringSSL's bindings, is not visible outside a few
/// AOSP modules; the functions come from the libcrypto the openssl crate links.
#[allow(non_camel_case_types)]
mod bssl_aead {
    #[repr(C)]
    pub struct EVP_AEAD {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct EVP_AEAD_CTX {
        _private: [u8; 0],
    }
    extern "C" {
        pub fn EVP_aead_aes_128_ccm_matter() -> *const EVP_AEAD;
        pub fn EVP_AEAD_CTX_new(
            aead: *const EVP_AEAD,
            key: *const u8,
            key_len: usize,
            tag_len: usize,
        ) -> *mut EVP_AEAD_CTX;
        pub fn EVP_AEAD_CTX_free(ctx: *mut EVP_AEAD_CTX);
        pub fn EVP_AEAD_CTX_seal(
            ctx: *const EVP_AEAD_CTX,
            out: *mut u8,
            out_len: *mut usize,
            max_out_len: usize,
            nonce: *const u8,
            nonce_len: usize,
            in_: *const u8,
            in_len: usize,
            ad: *const u8,
            ad_len: usize,
        ) -> i32;
        pub fn EVP_AEAD_CTX_open(
            ctx: *const EVP_AEAD_CTX,
            out: *mut u8,
            out_len: *mut usize,
            max_out_len: usize,
            nonce: *const u8,
            nonce_len: usize,
            in_: *const u8,
            in_len: usize,
            ad: *const u8,
            ad_len: usize,
        ) -> i32;
    }

    /// An AEAD context with a key, freed on drop.
    pub struct Ctx(pub *mut EVP_AEAD_CTX);

    impl Ctx {
        pub fn new(aead: &'static EVP_AEAD, key: &[u8], tag_len: usize) -> Option<Self> {
            let ctx = unsafe { EVP_AEAD_CTX_new(aead, key.as_ptr(), key.len(), tag_len) };
            (!ctx.is_null()).then_some(Self(ctx))
        }
    }

    impl Drop for Ctx {
        fn drop(&mut self) {
            unsafe { EVP_AEAD_CTX_free(self.0) }
        }
    }
}

impl<const KEY_LEN: usize, const NONCE_LEN: usize, const TAG_LEN: usize>
    Aead<KEY_LEN, NONCE_LEN, TAG_LEN>
{
    /// Create a new AEAD instance
    ///
    /// # Safety
    /// The caller must ensure that `aead` corresponds to the `KEY_LEN`, `NONCE_LEN` and
    /// `TAG_LEN` generic parameters.
    unsafe fn new(aead: &'static bssl_aead::EVP_AEAD) -> Self {
        Self(aead)
    }
}

impl<const KEY_LEN: usize, const NONCE_LEN: usize, const TAG_LEN: usize>
    crate::Aead<KEY_LEN, NONCE_LEN> for Aead<KEY_LEN, NONCE_LEN, TAG_LEN>
{
    fn encrypt_in_place<'a>(
        &mut self,
        key: CryptoSensitiveRef<'_, KEY_LEN>,
        nonce: CryptoSensitiveRef<'_, NONCE_LEN>,
        aad: &[u8],
        data: &'a mut [u8],
        data_len: usize,
    ) -> Result<&'a [u8], Error> {
        assert!(data_len + TAG_LEN <= data.len());

        let ctx = bssl_aead::Ctx::new(self.0, key.access(), TAG_LEN)
            .expect("EVP_AEAD_CTX_new failed");
        let mut out_len = 0;
        // BoringSSL allows the output to alias the input exactly: in place.
        let ok = unsafe {
            bssl_aead::EVP_AEAD_CTX_seal(
                ctx.0,
                data.as_mut_ptr(),
                &mut out_len,
                data.len(),
                nonce.access().as_ptr(),
                NONCE_LEN,
                data.as_ptr(),
                data_len,
                aad.as_ptr(),
                aad.len(),
            )
        };
        assert!(ok == 1, "EVP_AEAD_CTX_seal failed");

        Ok(&data[..out_len])
    }

    fn decrypt_in_place<'a>(
        &mut self,
        key: CryptoSensitiveRef<'_, KEY_LEN>,
        nonce: CryptoSensitiveRef<'_, NONCE_LEN>,
        aad: &[u8],
        data: &'a mut [u8],
    ) -> Result<&'a [u8], Error> {
        if data.len() < TAG_LEN {
            // Nothing to authenticate
            Err(ErrorCode::InvalidData)?;
        }

        let ctx = bssl_aead::Ctx::new(self.0, key.access(), TAG_LEN)
            .expect("EVP_AEAD_CTX_new failed");
        let mut out_len = 0;
        // A failure is a bad tag or ciphertext: untrusted input, an error, not a panic.
        let ok = unsafe {
            bssl_aead::EVP_AEAD_CTX_open(
                ctx.0,
                data.as_mut_ptr(),
                &mut out_len,
                data.len(),
                nonce.access().as_ptr(),
                NONCE_LEN,
                data.as_ptr(),
                data.len(),
                aad.as_ptr(),
                aad.len(),
            )
        };
        if ok != 1 {
            Err(ErrorCode::InvalidData)?;
        }

        Ok(&data[..out_len])
    }
}

/// Elliptic curve group implementation using OpenSSL
pub struct ECGroup<const LEN: usize, const SCALAR_LEN: usize> {
    /// The underlying OpenSSL EC group
    group: EcGroup,
}

impl<const LEN: usize, const SCALAR_LEN: usize> ECGroup<LEN, SCALAR_LEN> {
    /// Create a new EC group instance
    ///
    /// # Arguments
    /// - `nid`: The OpenSSL NID of the curve to use
    ///
    /// # Safety
    /// This function is unsafe because the caller must ensure that the
    /// `nid` parameter corresponds to the `LEN` and `SCALAR_LEN` generic parameters.
    unsafe fn new(nid: Nid) -> Result<Self, Error> {
        let group = openssl_check!(EcGroup::from_curve_name(nid))?;

        Ok(Self { group })
    }
}

/// An EC point implementation using OpenSSL
pub struct ECPoint<'a, const LEN: usize, const SCALAR_LEN: usize> {
    /// The associated EC group
    group: &'a ECGroup<LEN, SCALAR_LEN>,
    /// The underlying OpenSSL EC point
    point: EcPoint,
}

impl<const LEN: usize, const SCALAR_LEN: usize> ECPoint<'_, LEN, SCALAR_LEN> {
    /// Compute the OpenSSL EC key corresponding to this point
    fn ec_key(&self) -> Result<EcKey<Public>, Error> {
        openssl_check!(EcKey::from_public_key(&self.group.group, &self.point))
    }

    /// Write the point in canonical form
    fn write(&self, point: &mut CryptoSensitive<LEN>) -> Result<(), Error> {
        let mut ctx = openssl_check!(BigNumContext::new())?;

        let tmp = openssl_check!(self.point.to_bytes(
            &self.group.group,
            PointConversionForm::UNCOMPRESSED,
            &mut ctx,
        ))?;

        point.access_mut().copy_from_slice(tmp.as_slice());

        Ok(())
    }
}

impl<'a, const LEN: usize, const SCALAR_LEN: usize> crate::EcPoint<'a, LEN, SCALAR_LEN>
    for ECPoint<'a, LEN, SCALAR_LEN>
{
    type Scalar<'s>
        = ECScalar<'s, SCALAR_LEN, LEN>
    where
        Self: 'a + 's;

    fn is_valid_pubkey(&self) -> Result<bool, Error> {
        // RFC-9383 public-key validation: on the curve and not the identity
        // (point at infinity). `is_on_curve` also rejects out-of-range
        // coordinates.
        let mut ctx = openssl_check!(BigNumContext::new())?;

        let on_curve = openssl_check!(self.point.is_on_curve(&self.group.group, &mut ctx))?;
        let infinity = self.point.is_infinity(&self.group.group);

        Ok(on_curve && !infinity)
    }

    fn neg(&self) -> Result<Self, Error> {
        let mut result = openssl_check!(self.point.to_owned(&self.group.group))?;

        let ctx = openssl_check!(BigNumContext::new())?;
        openssl_check!(result.invert(&self.group.group, &ctx))?;

        Ok(Self {
            group: self.group,
            point: result,
        })
    }

    fn mul(&self, scalar: &Self::Scalar<'a>) -> Result<Self, Error> {
        let mut result = openssl_check!(EcPoint::new(&self.group.group))?;

        let ctx = openssl_check!(BigNumContext::new())?;

        openssl_check!(result.mul(&self.group.group, &self.point, &scalar.scalar, &ctx))?;

        Ok(Self {
            group: self.group,
            point: result,
        })
    }

    fn add_mul(
        &self,
        s1: &Self::Scalar<'a>,
        p2: &Self,
        s2: &Self::Scalar<'a>,
    ) -> Result<Self, Error> {
        let a = self.mul(s1)?;
        let b = p2.mul(s2)?;

        let mut ctx = openssl_check!(BigNumContext::new())?;

        let mut result = openssl_check!(EcPoint::new(&self.group.group))?;

        openssl_check!(result.add(&self.group.group, &a.point, &b.point, &mut ctx))?;

        Ok(Self {
            group: self.group,
            point: result,
        })
    }

    fn write_canon(&self, point: &mut CryptoSensitive<LEN>) -> Result<(), Error> {
        self.write(point)
    }
}

impl<'a, const KEY_LEN: usize, const SECRET_KEY_LEN: usize, const SIGNATURE_LEN: usize>
    crate::PublicKey<'a, KEY_LEN, SIGNATURE_LEN> for ECPoint<'a, KEY_LEN, SECRET_KEY_LEN>
{
    fn verify(
        &self,
        data: &[u8],
        signature: CryptoSensitiveRef<'_, SIGNATURE_LEN>,
    ) -> Result<bool, Error> {
        // First get the SHA256 of the message
        let mut hasher = openssl_check!(Hasher::new(MessageDigest::sha256()))?;
        openssl_check!(hasher.update(data))?;
        let digest = openssl_check!(hasher.finish())?;

        let r = openssl_check!(BigNum::from_slice(&signature.access()[..SIGNATURE_LEN / 2]))?;
        let s = openssl_check!(BigNum::from_slice(&signature.access()[SIGNATURE_LEN / 2..]))?;
        let sig = openssl_check!(EcdsaSig::from_private_components(r, s))?;

        let ec_key = openssl_check!(EcKey::from_public_key(&self.group.group, &self.point))?;

        openssl_check!(sig.verify(&digest, &ec_key))
    }

    fn write_canon(&self, key: &mut CryptoSensitive<KEY_LEN>) -> Result<(), Error> {
        self.write(key)
    }
}

/// An EC scalar implementation using OpenSSL
pub struct ECScalar<'a, const LEN: usize, const POINT_LEN: usize> {
    /// The associated EC group
    group: &'a ECGroup<POINT_LEN, LEN>,
    /// The underlying OpenSSL big number scalar
    scalar: BigNum,
}

impl<const LEN: usize, const POINT_LEN: usize> ECScalar<'_, LEN, POINT_LEN> {
    /// Compute the OpenSSL EC point corresponding to this scalar
    fn ec_pub_key_point(&self) -> Result<EcPoint, Error> {
        let ctx = openssl_check!(BigNumContext::new())?;

        let mut point = openssl_check!(EcPoint::new(&self.group.group))?;

        openssl_check!(point.mul(
            &self.group.group,
            self.group.group.generator(),
            &self.scalar,
            &ctx,
        ))?;

        Ok(point)
    }

    /// Compute the OpenSSL EC key corresponding to this scalar
    fn ec_key(&self) -> Result<EcKey<Private>, Error> {
        let point = self.ec_pub_key_point()?;

        openssl_check!(EcKey::from_private_components(
            &self.group.group,
            &self.scalar,
            &point,
        ))
    }

    /// Write the scalar in canonical form
    fn write(&self, scalar: &mut CryptoSensitive<LEN>) -> Result<(), Error> {
        scalar
            .access_mut()
            .copy_from_slice(openssl_check!(self.scalar.to_vec_padded(LEN as _))?.as_slice());

        Ok(())
    }
}

impl<'a, const LEN: usize, const POINT_LEN: usize> crate::EcScalar<'a, LEN>
    for ECScalar<'a, LEN, POINT_LEN>
{
    fn mul(&self, other: &Self) -> Result<Self, Error> {
        Ok(Self {
            group: self.group,
            scalar: self.scalar.mul(&other.scalar),
        })
    }

    fn write_canon(&self, scalar: &mut CryptoSensitive<LEN>) -> Result<(), Error> {
        self.write(scalar)
    }
}

impl<'a, const LEN: usize, const POINT_LEN: usize, const SIGNATURE_LEN: usize>
    crate::SigningSecretKey<'a, POINT_LEN, SIGNATURE_LEN> for ECScalar<'a, LEN, POINT_LEN>
{
    type PublicKey<'s>
        = ECPoint<'s, POINT_LEN, LEN>
    where
        Self: 's;

    fn csr<'s>(&self, buf: &'s mut [u8]) -> Result<&'s [u8], Error> {
        let mut builder = openssl_check!(X509ReqBuilder::new())?;
        openssl_check!(builder.set_version(0))?;

        let pkey = openssl_check!(PKey::from_ec_key(self.ec_key()?))?;
        openssl_check!(builder.set_pubkey(&pkey))?;

        let mut name_builder = openssl_check!(X509NameBuilder::new())?;
        openssl_check!(name_builder.append_entry_by_text_with_type(
            "O",
            "CSR",
            Asn1Type::IA5STRING
        ))?;
        let subject_name = name_builder.build();
        openssl_check!(builder.set_subject_name(&subject_name))?;

        openssl_check!(builder.sign(&pkey, MessageDigest::sha256()))?;

        let csr = openssl_check!(builder.build().to_der())?;
        if csr.len() <= buf.len() {
            buf[..csr.len()].copy_from_slice(csr.as_slice());

            Ok(&buf[..csr.len()])
        } else {
            Err(ErrorCode::NoSpace.into())
        }
    }

    fn pub_key(&self) -> Result<Self::PublicKey<'a>, Error> {
        let ctx = openssl_check!(BigNumContext::new())?;

        let mut pub_key = Self::PublicKey {
            group: self.group,
            point: openssl_check!(EcPoint::new(&self.group.group))?,
        };

        openssl_check!(pub_key.point.mul(
            &self.group.group,
            self.group.group.generator(),
            &self.scalar,
            &ctx,
        ))?;

        Ok(pub_key)
    }

    fn sign(
        &self,
        data: &[u8],
        signature: &mut CryptoSensitive<SIGNATURE_LEN>,
    ) -> Result<(), Error> {
        // First get the SHA256 of the message
        let mut hasher = openssl_check!(Hasher::new(MessageDigest::sha256()))?;
        openssl_check!(hasher.update(data))?;
        let digest = openssl_check!(hasher.finish())?;

        let our_ec_key = self.ec_key()?;

        let sig = openssl_check!(EcdsaSig::sign(&digest, &our_ec_key))?;

        signature.access_mut()[..crate::PKC_SIGNATURE_LEN / 2].copy_from_slice(
            openssl_check!(sig.r().to_vec_padded(crate::PKC_SIGNATURE_LEN as i32 / 2))?.as_slice(),
        );
        signature.access_mut()[crate::PKC_SIGNATURE_LEN / 2..].copy_from_slice(
            openssl_check!(sig.s().to_vec_padded(crate::PKC_SIGNATURE_LEN as i32 / 2))?.as_slice(),
        );

        Ok(())
    }
}

impl<
        'a,
        const LEN: usize,
        const POINT_LEN: usize,
        const SIGNATURE_LEN: usize,
        const SHARED_SECRET_LEN: usize,
    > crate::SecretKey<'a, LEN, POINT_LEN, SIGNATURE_LEN, SHARED_SECRET_LEN>
    for ECScalar<'a, LEN, POINT_LEN>
{
    fn derive_shared_secret(
        &self,
        peer_pub_key: &Self::PublicKey<'a>,
        shared_secret: &mut CryptoSensitive<SHARED_SECRET_LEN>,
    ) -> Result<(), Error> {
        let our_priv_key = openssl_check!(PKey::from_ec_key(self.ec_key()?))?;
        let peer_pub_key = openssl_check!(PKey::from_ec_key(peer_pub_key.ec_key()?))?;

        let mut deriver = openssl_check!(Deriver::new(&our_priv_key))?;

        openssl_check!(deriver.set_peer(&peer_pub_key))?;
        openssl_check!(deriver.derive(shared_secret.access_mut()))?;

        Ok(())
    }

    fn write_canon(&self, key: &mut CryptoSensitive<LEN>) -> Result<(), Error> {
        self.write(key)
    }
}

fn map_err(msg: &str, _err: openssl::error::ErrorStack) -> Error {
    #[cfg(feature = "log")]
    error!("{}: {}", msg, _err);

    #[cfg(feature = "defmt")]
    error!("{}", msg);

    ErrorCode::InvalidData.into()
}
