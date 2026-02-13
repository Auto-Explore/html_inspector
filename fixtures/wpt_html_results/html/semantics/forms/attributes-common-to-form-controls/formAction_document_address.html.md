# html/semantics/forms/attributes-common-to-form-controls/formAction_document_address.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/attributes-common-to-form-controls/formAction_document_address.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>HTML Test: formAction_document_address</title>
    <link rel="author" title="Intel" href="http://www.intel.com/">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-fs-formaction">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-document's-address">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-button-element">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-input-element">
    <meta name="assert" content="On getting the formAction IDL attribute, when the content attribute is missing or its value is the empty string, the document's address must be returned instead.">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>

    <div id="missing" style="display:none">
      <button type="submit">Submit</button>
      <input type="submit">
    </div>

    <div id="empty_string" style="display:none">
      <button type="submit" formaction="">Submit</button>
      <input type="submit" formaction="">
    </div>

    <div id="no_assigned_value" style="display:none">
      <button type="submit" formaction>Submit</button>
      <input type="submit" formaction>
    </div>

    <script>
      // formaction content attribute is missing
      test(function() {
        var formAction = document.querySelector('#missing button').formAction;
        var address = document.location.href;
        assert_equals(formAction, address);
      }, "Check if button.formAction is the document's address when formaction content attribute is missing");

      test(function() {
        var formAction = document.querySelector('#missing input').formAction;
        var address = document.location.href;
        assert_equals(formAction, address);
      }, "Check if input.formAction is the document's address when formaction content attribute is missing");

      // formaction content attribute value is empty string
      test(function() {
        var formAction = document.querySelector('#empty_string button').formAction;
        var address = document.location.href;
        assert_equals(formAction, address);
      }, "Check if button.formAction is the document's address when formaction content attribute value is empty string");

      test(function() {
        var formAction = document.querySelector('#empty_string input').formAction;
        var address = document.location.href;
        assert_equals(formAction, address);
      }, "Check if input.formAction is the document's address when formaction content attribute value is empty string");

      // formaction content attribute value is not assigned, just for comparison with empty string above
      test(function() {
        var formAction = document.querySelector('#no_assigned_value button').formAction;
        var address = document.location.href;
        assert_equals(formAction, address);
      }, "Check if button.formAction is the document's address when formaction content attribute value is not assigned");

      test(function() {
        var formAction = document.querySelector('#no_assigned_value input').formAction;
        var address = document.location.href;
        assert_equals(formAction, address);
      }, "Check if input.formAction is the document's address when formaction content attribute value is not assigned");

      var newUrl = location.href.replace(/\/[^\/]*$/,'\/dummy.html');
      history.pushState('','','dummy.html');

      test(function() {
        assert_equals(document.location.href, newUrl);

        var formAction = document.querySelector('#missing button').formAction;
        var address = document.location.href;
        assert_equals(formAction, address);
      }, "Check if button.formAction is the document's new address when formaction content attribute is missing and pushState has been used");

      test(function() {
        assert_equals(document.location.href, newUrl);

        var formAction = document.querySelector('#missing input').formAction;
        var address = document.location.href;
        assert_equals(formAction, address);
      }, "Check if input.formAction is the document's new address when formaction content attribute is missing and pushState has been used");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.button.formaction.empty",
      "message": "Bad value “” for attribute “formaction” on element “button”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1113,
        "byte_start": 1077,
        "col": 7,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.input.formaction.empty",
      "message": "Bad value “” for attribute “formaction” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1170,
        "byte_start": 1135,
        "col": 7,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.button.formaction.empty",
      "message": "Bad value “” for attribute “formaction” on element “button”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1276,
        "byte_start": 1243,
        "col": 7,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.input.formaction.empty",
      "message": "Bad value “” for attribute “formaction” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1330,
        "byte_start": 1298,
        "col": 7,
        "line": 30
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
  "source_name": "html/semantics/forms/attributes-common-to-form-controls/formAction_document_address.html"
}
```
