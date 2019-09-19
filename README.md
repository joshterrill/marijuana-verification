# Marijuana Verification API

A Marijuana Verification API written in Rust that verifies patients for the following systems:

*   Green Life Medical
*   Patient ID Center

If you would like more systems added or you would like this API implemented on your website, email me at [joshterrill.dev@gmail.com](mailto:joshterrill.dev@gmail.com).

Demo: [https://marijuanaverification.herokuapp.com](https://marijuanaverification.herokuapp.com)

### Endpoints

**Green Life Medical**

```
Method: GET
URL: /greenlife/<rec_id>
Example Response:
  {
    "doctors_name": "HARNAN GRETTA, M.D.",
    "issue_date": "2017-10-13",
    "patient_initials": "J.A.G",
    "status": "EXPIRED",
    "valid_through": "2018-10-12"
  }
```

**Patient ID Center**

```
Method: GET
URL: /patientidcenter/<member_number>/<id_number>
Example Response:
  {
    "status": "ACTIVE"
  }
```

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy?template=https://github.com/joshterrill/marijuana-verification)