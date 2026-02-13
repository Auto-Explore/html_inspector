# html/semantics/scripting-1/the-script-element/module/module-import-referrer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/module-import-referrer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Referrer for module imports</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>setup({ explicit_done: true })</script>
</head>
<body>
<script type="module">

import { referrerExternalStatic, referrerExternalDynamic } from "./module-import-referrer.js";

// "name" parameter is necessary for bypassing the module map.
import { referrer as referrerInlineStatic } from "./resources/referrer-checker.py?name=internal-static"
const { referrer: referrerInlineDynamic } = await import("./resources/referrer-checker.py?name=internal-dynamic");

const scriptURL = new URL("module-import-referrer.js", location.href)

test(t => {
  assert_equals(
    referrerInlineStatic, location.href,
      "Referrer should be the document URL");
}, "Static imports from inline modules in the HTML document");

test(t => {
  assert_equals(
    referrerInlineDynamic, location.href,
      "Referrer should be the document URL");
}, "Dynamic imports from inline modules in the HTML document");

test(t => {
  assert_equals(
    referrerExternalStatic, scriptURL.href,
      "Referrer should be the importer module URL");
}, "Static imports from external modules");

test(t => {
  assert_equals(
    referrerExternalDynamic, scriptURL.href,
      "Referrer should be the document URL");
}, "Dynamic imports from external modules");

done();

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
  "source_name": "html/semantics/scripting-1/the-script-element/module/module-import-referrer.html"
}
```
