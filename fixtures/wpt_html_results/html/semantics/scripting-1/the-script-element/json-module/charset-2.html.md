# html/semantics/scripting-1/the-script-element/json-module/charset-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/charset-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="windows-1250">
<title>JSON modules: UTF-8 decoding</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script type="module">
  import json from "../serve-with-content-type.py?fn=json-module/utf-8.json" with { type: "json"};
  test(() => {
    assert_equals(json.data, "śćążź");
  }, "JSON module should be loaded as utf-8 even though document's encoding is windows-1250");
</script>
<script type="module">
  import json from "../serve-with-content-type.py?fn=json-module/windows-1250.json&ct=text/json%3Bcharset=windows-1250" with { type: "json"};
  test(() => {
    assert_not_equals(json.data, "śćążź",
                    'Should be decoded as UTF-8');
  }, "JSON module should be loaded as utf-8 even if it is encoded in windows-1250 and served with a windows-1250 charset response header, and this document's encoding is windows-1250");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1250” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 45,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/charset-2.html"
}
```
