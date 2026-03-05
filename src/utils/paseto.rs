use pasetors::{
    public, 
    Public,
    claims::{Claims, ClaimsValidationRules}, 
    keys::{AsymmetricPublicKey, AsymmetricSecretKey}, 
    public::sign, 
    token::UntrustedToken, 
    version4::V4
};
use std::env;

use crate::{entities::user::Model, utils::errors::CustomError};


fn get_auth_secret_key() -> Result<AsymmetricSecretKey<V4>, CustomError> {
    let secret_key = env::var("ACCESS_PRIVATE_KEY").expect("Secret key not set");
    let secret_key_byte = hex::decode(secret_key).unwrap();
    let secret = AsymmetricSecretKey::<V4>::from(&secret_key_byte)?;
    Ok(secret)
}

fn get_auth_public_key() -> Result<AsymmetricPublicKey<V4>, CustomError> {
    let public_key = env::var("ACCESS_PUBLIC_KEY").expect("Public key not set");
    let public_key_byte = hex::decode(public_key).unwrap();
    let public = AsymmetricPublicKey::<V4>::from(&public_key_byte)?;
    Ok(public)
}


pub fn create_token(user: &Model) -> Result<String, CustomError> {
    let secret_key = get_auth_secret_key()?;
    let mut claims = Claims::new()?;
    claims.add_additional("email", user.email.clone())?;

    // generate token
    let pub_token = sign(&secret_key, &claims, None, None)?;
    Ok(pub_token)
}

pub fn verify_token(token: &String) -> Result<Claims, CustomError> {
    let public_key = get_auth_public_key()?;
    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Public,V4>::try_from(token)?;
    let trusted_token = public::verify(&public_key, &untrusted_token, &validation_rules, None, None)?;

    let claims = trusted_token.payload_claims().unwrap();

    Ok(claims.clone())
    
}