use rust_utils::add;
use rust_utils::decrypt_data;
use rust_utils::encrypt_data;

#[test]
fn test_add() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_encrypt_data() -> Result<(), Box<dyn std::error::Error>> {
    let private_pem_str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpQIBAAKCAQEA4gRE9HOERcEUhKSNgqYtnVW9LIy+b5qM+jTEDoi956DhTytI
Ax+phOOrC/cI68+XXnPFZsNHy7ZxC2nONEzNYuS7ev9qxAxhhoRYLXDhsuqVsPg8
KuJAJRSDAhpdcw39vij4nccnlAjjFye73Qo7Mb7Gd7YAFBtX81I/u58QZrTqHVwk
pqZNWB+2YK//XV35PcOquKE4K9qyQfAydMKEmDjj6Q1yj6XoWJlaMspG/WRCUMM6
G52JP0Ln2gG3F5wFIy3jbYDq7AOVHkzX+ZHmlbCzceOVWBsBtfy8sjxOfFcdVGHO
KgQEP6fuBV/ohqqKDCwAGoA2RzIdkjtY6msWlwIDAQABAoIBAQCTDjYTW0nX3N3N
xAcy147cTNnuL5EEqmlJrqVV05DEMcAHM+EB70rr5Pn4qf4K+CHD2c25pBuFpyn8
RBZfnw0PaEW3rJI6Btg5nsiEoGqpb8p/v7PeLyGPPr5S0va0kojjfmLsvZS50+J1
wWuRUUPchQU7BJcefgAGmCBRohgYyipAAMiTH2U8z00LyFI9QU5xGybP+GOoPn59
P7QnEa4/Tn/+6bY2jcanI3JzznZL1gvvxx+lSurZIapl+P91TXiXQ/AJEJ7TPTDc
Riv83iBevzlxKyTSo8oGjLKK6Ml/Sh0Zxasf+812G+E9FFIBkkhInQJX0FwMlvHZ
YoSiWM1JAoGBAOegHN2Rcp3xP/S4KR5Pd7Knjv3pZ+4awF3/ffUGdIzK3dHGKQy3
YvT8By/QrhjeArpWrmmTlWup7FnSFzFfUzyxWjgXSIQDTIRijvDq0ZtlKU8UHYPN
7ePQl412QPP+LtCt6+Yd8AuvHCnfyisSYy35anwtr7AkelEtz8R7hqFbAoGBAPnN
DxnphprZqnaZxI3YxrqDlcKKtA81qG4i/HVtjJJdf3sedQ9imlHoMLq5/1mnKaB8
E7/YR1Ib0OAN73LfapDh/sFhzaiPM37g+2VflUU5BToDtDnQMt0/RV7t6jd8O4tu
QZLVgXApwY507mmyz4W+taiQ7+M9bAxXO+3VcYh1AoGBAI7WZ1af3l3WK4mflAPU
H821lPGyYVwtdRnCeAuFWpSEejxmBmSIJudLEKeE+gftySLeV5pV39xQIqfVbmYN
Egiomili+l4mpqYxHVMmi/JXdR0GG5lvgdduiDc9iJqu0nHv/zyek6yw5R5Rmpvr
L+xnFirUBbcLF78+EBVr079nAoGAD86E/RvE07mgSr7yLBOih5zZ9iR2vluj28xE
811KPtzBu1WzDJUttK8fnkE0wkSMosYXLdWOtchi0DqxgzBV+vMB/tSkgd0F4ip0
XfbNaELybLhdSCc/gLaHOjmNz5MB5ZHFfngaJ7HMuKn3iCKzdQAbWJ5LP7LcSm+e
sC8Ibx0CgYEAg2AGQd2FFvekl4LU+vho5nmJ+ieDeWMzEW9kY5Gv3UfvSkJCXgNL
seTQ1kWIIiQE6Yc9xT/FSs3YWC9YuUK5DMog0bH+xnqFxc1vVqMtR+8Khf5BhkVC
eY7i0K6c9dKEiAWBsvd3C8/ktcXSps8wjxGVH+X/2Re316biQfk6QV8=
-----END RSA PRIVATE KEY-----"#;

    let public_pem_str = r#"-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEA4gRE9HOERcEUhKSNgqYtnVW9LIy+b5qM+jTEDoi956DhTytIAx+p
hOOrC/cI68+XXnPFZsNHy7ZxC2nONEzNYuS7ev9qxAxhhoRYLXDhsuqVsPg8KuJA
JRSDAhpdcw39vij4nccnlAjjFye73Qo7Mb7Gd7YAFBtX81I/u58QZrTqHVwkpqZN
WB+2YK//XV35PcOquKE4K9qyQfAydMKEmDjj6Q1yj6XoWJlaMspG/WRCUMM6G52J
P0Ln2gG3F5wFIy3jbYDq7AOVHkzX+ZHmlbCzceOVWBsBtfy8sjxOfFcdVGHOKgQE
P6fuBV/ohqqKDCwAGoA2RzIdkjtY6msWlwIDAQAB
-----END RSA PUBLIC KEY-----"#;

    let data = "password";

    // 调用加密函数
    let encrypted_data = encrypt_data(public_pem_str, data);
    let encrypted_string = encrypted_data?;
    let encrypted_str = encrypted_string.as_str();
    eprintln!("encrpted str:{}", encrypted_str);

    let decrypted_data = decrypt_data(private_pem_str, encrypted_str);

    assert_eq!(decrypted_data?.as_str(), data);
    Ok(())
}
