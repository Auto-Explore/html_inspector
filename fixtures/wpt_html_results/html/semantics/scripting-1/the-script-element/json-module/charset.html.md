# html/semantics/scripting-1/the-script-element/json-module/charset.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/charset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>JSON modules: UTF-8 decoding</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script type="module" onerror="unreachable()">
  import json from "../serve-with-content-type.py?fn=json-module/utf-8.json&ct=text/json%3Bcharset=utf-8" with { type: "json"};
  test(() => {
    assert_equals(json.data, "śćążź");
  }, "JSON module should be loaded as utf-8 when charset=utf8 is specified");
</script>
<script type="module" onerror="unreachable()">
  import json from "../serve-with-content-type.py?fn=json-module/utf-8.json&ct=text/json%3Bcharset=shift-jis" with { type: "json"};
  test(() => {
    assert_equals(json.data, "śćążź");
  }, "JSON module should be loaded as utf-8 when charset=shift-jis is specified");
</script>
<script type="module" onerror="unreachable()">
  import json from "../serve-with-content-type.py?fn=json-module/utf-8.json&ct=text/json%3Bcharset=windows-1252" with { type: "json"};
  test(() => {
    assert_equals(json.data, "śćążź");
  }, "JSON module should be loaded as utf-8 when charset=windows-1252 is specified");
</script>
<script type="module" onerror="unreachable()">
  import json from "../serve-with-content-type.py?fn=json-module/utf-8.json&ct=text/json%3Bcharset=utf-7" with { type: "json"};;
  test(() => {
    assert_equals(json.data, "śćążź");
  }, "JSON module should be loaded as utf-8 when charset=utf-7 is specified");
</script>
<script type="module" onerror="unreachable()">
  import json from "../serve-with-content-type.py?fn=json-module/windows-1250.json&ct=text/json%3Bcharset=windows-1250" with { type: "json"};
  test(() => {
    assert_not_equals(json.data, "śćążź",
                    'Should be decoded as UTF-8');
  }, "JSON module should be loaded as utf-8 even if it is encoded in windows-1250 and served with a windows-1250 charset response header");
</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/charset.html"
}
```
