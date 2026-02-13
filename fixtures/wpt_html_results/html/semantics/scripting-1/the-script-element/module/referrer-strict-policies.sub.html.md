# html/semantics/scripting-1/the-script-element/module/referrer-strict-policies.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/referrer-strict-policies.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Referrer with the strict-origin referrer policy</title>
<meta name="referrer" content="strict-origin">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script type="module">

// "name" parameter is necessary for bypassing the module map in descendant import.

import { referrer as insecureImport } from "./resources/import-referrer-checker-insecure.sub.js?name=insecure_import";
import { referrer as secureImport } from "https://{{domains[www]}}:{{ports[https][0]}}/html/semantics/scripting-1/the-script-element/module/resources/import-referrer-checker-insecure.sub.js?name=secure_import";

const origin = (new URL(location.href)).origin + "/";

test(t => {
  assert_equals(
      insecureImport, origin,
      "A document with the strict-origin referrer policy served over HTTP, " +
      "imports an module script over HTTP, that imports a descendant script " +
      "over HTTP. The request for the descendant script is sent with a " +
      "`Referer` header with the page's origin");

  assert_equals(
      secureImport, "",
      "A document with the strict-origin referrer policy served over HTTP, " +
      "imports an module script over HTTPS, that imports a descendant script " +
      "over HTTP. The request for the descendant script is sent with no " +
      "`Referer` header");
}, "The strict-* referrer policies compare the trustworthiness of a " +
   "request's referrer string against its URL");

</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/referrer-strict-policies.sub.html"
}
```
