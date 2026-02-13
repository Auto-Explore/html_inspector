# html/semantics/scripting-1/the-script-element/module/dynamic-import/alpha/base-url.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/alpha/base-url.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Base URLs used in resolving specifiers in dynamic imports</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
self.testName = "same origin classic <script>";
self.baseUrlSanitized = false;
</script>
<script src="../beta/redirect.py?location=http://{{host}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/dynamic-import/gamma/base-url.sub.js"></script>

<script>
self.testName = "cross origin classic <script> without crossorigin attribute";
self.baseUrlSanitized = true;
</script>
<script src="../beta/redirect.py?location=http://{{hosts[alt][]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/dynamic-import/gamma/base-url.sub.js"></script>

<script>
self.testName = "cross origin classic <script> with crossorigin attribute";
self.baseUrlSanitized = false;
</script>
<script src="../beta/redirect.py?location=http://{{hosts[alt][]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/dynamic-import/gamma/base-url.sub.js%3Fpipe=header(Access-Control-Allow-Origin,*)" crossorigin></script>

<script>
self.testName = "cross origin module <script>";
self.baseUrlSanitized = false;
</script>
<script src="../beta/redirect.py?location=http://{{hosts[alt][]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/dynamic-import/gamma/base-url.sub.js%3Fpipe=header(Access-Control-Allow-Origin,*)" type="module"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/alpha/base-url.sub.html"
}
```
