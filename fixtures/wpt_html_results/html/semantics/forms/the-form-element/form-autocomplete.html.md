# html/semantics/forms/the-form-element/form-autocomplete.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-autocomplete.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>form autocomplete attribute</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#the-form-element">
<link rel=help href="https://html.spec.whatwg.org/multipage/#attr-fe-autocomplete">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<form name="missing_attribute">
  <input>
  <input autocomplete="on">
  <input autocomplete="off">
  <input autocomplete="foobar">
</form>
<form name="autocomplete_on" autocomplete="on">
  <input>
  <input autocomplete="on">
  <input autocomplete="off">
  <input autocomplete="foobar">
</form>
<form name="autocomplete_off" autocomplete="off">
  <input>
  <input autocomplete="on">
  <input autocomplete="off">
  <input autocomplete="foobar">
</form>
<form name="autocomplete_invalid" autocomplete="foobar">
  <input>
  <input autocomplete="on">
  <input autocomplete="off">
  <input autocomplete="foobar">
</form>
<script>
  function autocompletetest(form, expectedValues, desc) {
    test(function(){
      assert_equals(form.autocomplete, expectedValues[0]);
      assert_equals(form.elements[0].autocomplete, expectedValues[1]);
      assert_equals(form.elements[1].autocomplete, expectedValues[2]);
      assert_equals(form.elements[2].autocomplete, expectedValues[3]);
      assert_equals(form.elements[3].autocomplete, expectedValues[4]);
    }, desc);
  }

  autocompletetest(document.forms.missing_attribute, ["on", "", "on", "off", ""], "form autocomplete attribute missing");
  autocompletetest(document.forms.autocomplete_on, ["on", "", "on", "off", ""], "form autocomplete attribute on");
  autocompletetest(document.forms.autocomplete_off, ["off", "", "on", "off", ""], "form autocomplete attribute off");
  autocompletetest(document.forms.autocomplete_invalid, ["on", "", "on", "off", ""], "form autocomplete attribute invalid");

  var keywords = [ "on", "off", "name", "honorific-prefix", "given-name", "additional-name", "family-name", "honorific-suffix", "nickname", "username", "new-password", "current-password", "one-time-code", "organization-title", "organization", "street-address", "address-line1", "address-line2", "address-line3", "address-level4", "address-level3", "address-level2", "address-level1", "country", "country-name", "postal-code", "cc-name", "cc-given-name", "cc-additional-name", "cc-family-name", "cc-number", "cc-exp", "cc-exp-month", "cc-exp-year", "cc-csc", "cc-type", "transaction-currency", "transaction-amount", "language", "bday", "bday-day", "bday-month", "bday-year", "sex", "url", "photo", "tel", "tel-country-code", "tel-national", "tel-area-code", "tel-local", "tel-local-prefix", "tel-local-suffix", "tel-extension", "email", "impp", "webauthn" ];

  keywords.forEach(function(keyword) {
    test(function(){
      var input = document.createElement("input");
      // Include whitespace to test splitting tokens on whitespace.
      // Convert to uppercase to ensure that the tokens are normalized to lowercase.
      input.setAttribute("autocomplete", " " + keyword.toUpperCase() + "\t");
      assert_equals(input.autocomplete, keyword);
    }, keyword + " is an allowed autocomplete field name");
  });


test(() => {
  const select = document.createElement("select");
  select.setAttribute("autocomplete", "  \n");
  assert_equals(select.autocomplete, "");
}, "Test whitespace-only attribute value");

test(() => {
  const select = document.createElement("select");

  select.setAttribute("autocomplete", "foo off");
  assert_equals(select.autocomplete, "");

  // Normal category; max=3
  select.setAttribute("autocomplete", "foo section-foo billing name");
  assert_equals(select.autocomplete, "");

  // Contact category; max=4
  select.setAttribute("autocomplete", "foo section-bar billing work tel");
  assert_equals(select.autocomplete, "");

  // Credential category; max=5
  select.setAttribute("autocomplete", "foo section-bar billing work tel webauthn");
  assert_equals(select.autocomplete, "");
}, "Test maximum number of tokens");

test(() => {
  const textarea = document.createElement("textarea");

  textarea.setAttribute("autocomplete", "call-sign");
  assert_equals(textarea.autocomplete, "");
}, "Unknown field");

test(() => {
  const hidden = document.createElement("input");
  hidden.type = "hidden";
  hidden.setAttribute("autocomplete", "on");
  assert_equals(hidden.autocomplete, "");
  hidden.setAttribute("autocomplete", "off");
  assert_equals(hidden.autocomplete, "");
}, "Test 'wearing the autofill anchor mantle' with off/on");

test(() => {
  const textarea = document.createElement("textarea");

  textarea.setAttribute("autocomplete", " HOME\ntel");
  assert_equals(textarea.autocomplete, "home tel");

  textarea.setAttribute("autocomplete", "shipping   country");
  assert_equals(textarea.autocomplete, "shipping country");

  textarea.setAttribute("autocomplete", "billing  work  email");
  assert_equals(textarea.autocomplete, "billing work email");

  textarea.setAttribute("autocomplete", "  section-FOO  bday");
  assert_equals(textarea.autocomplete, "section-foo bday");
}, "Serialize combinations of section, mode, contact, and field");

test(() => {
  const textarea = document.createElement("textarea");

  textarea.setAttribute("autocomplete", "\tusername webauthn");
  assert_equals(textarea.autocomplete, "username webauthn");

  textarea.setAttribute("autocomplete", "  section-LOGIN  shipping work tel webauthn ");
  assert_equals(textarea.autocomplete, "section-login shipping work tel webauthn");
}, "Serialize combinations of section, mode, contact, field, and credential");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.autocomplete.invalid",
      "message": "Bad value “foobar” for attribute “autocomplete” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 570,
        "byte_start": 541,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.autocomplete.invalid",
      "message": "Bad value “foobar” for attribute “autocomplete” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 725,
        "byte_start": 696,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.autocomplete.invalid",
      "message": "Bad value “foobar” for attribute “autocomplete” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 882,
        "byte_start": 853,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.autocomplete.invalid",
      "message": "Bad value “foobar” for attribute “autocomplete” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1046,
        "byte_start": 1017,
        "col": 3,
        "line": 32
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-form-element/form-autocomplete.html"
}
```
