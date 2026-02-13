# html/semantics/forms/attributes-common-to-form-controls/formaction.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/attributes-common-to-form-controls/formaction.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html><head>
        <title>formaction on button element</title>
        <meta content="text/html; charset=UTF-8" http-equiv="content-type">
        <meta content="formaction on button element" name="description">
    <link href="https://html.spec.whatwg.org/multipage/#dom-fs-formaction" rel="help">
</head>
    <body>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>

        <div id="log"></div>
        <button formaction="http://www.example.com/" style="display: none" type="submit">Submit</button>
        <input formaction="http://www.example.com/" style="display: none" type="submit" value="submit">
        <input style="display: none" type="submit" value="submit">
        <input formaction="" style="display: none" type="submit" value="submit">

        <script type="text/javascript">
        function relativeToAbsolute(relativeURL) {
          var a = document.createElement('a');
          a.href = relativeURL;
          return a.href;
        }
        test(function() {assert_equals(document.getElementsByTagName("button")[0].formAction, "http://www.example.com/")}, "formAction on button support");
        test(function() {assert_equals(document.getElementsByTagName("input")[0].formAction, "http://www.example.com/")}, "formAction on input support");

        var testElem = document.getElementsByTagName("input")[0];
        testElem.formAction = "http://www.example.com/page2.html";

        test(function() {assert_equals(document.getElementsByTagName("input")[0].formAction, "http://www.example.com/page2.html")}, "formaction absolute URL value on input reflects correct value after being updated by the DOM");
        test(function() {assert_equals(document.getElementsByTagName("input")[0].getAttribute("formaction"), "http://www.example.com/page2.html")}, "formAction absolute URL value is correct using getAttribute");

        var testElem = document.getElementsByTagName("input")[0];
        testElem.formAction = "../page3.html";

        test(function() {assert_equals(document.getElementsByTagName("input")[0].formAction, relativeToAbsolute('../page3.html'))}, "formAction relative URL value on input reflects correct value after being updated by the DOM");
        test(function() {assert_equals(document.getElementsByTagName("input")[0].getAttribute("formaction"), "../page3.html")}, "formAction relative URL value is correct using getAttribute");

        test(function() {assert_equals(document.getElementsByTagName("input")[1].formAction, document.URL)}, "On getting, when formaction is missing, the document's address must be returned");
        test(function() {assert_equals(document.getElementsByTagName("input")[2].formAction, document.URL)}, "On getting, when formaction value is the empty string, the document's address must be returned");
        </script>
</body></html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.formaction.empty",
      "message": "Bad value “” for attribute “formaction” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 844,
        "byte_start": 772,
        "col": 9,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 885,
        "byte_start": 854,
        "col": 9,
        "line": 18
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
  "source_name": "html/semantics/forms/attributes-common-to-form-controls/formaction.html"
}
```
