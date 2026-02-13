# html/semantics/scripting-1/the-script-element/css-module/charset.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/css-module/charset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>CSS modules: UTF-8 decoding</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script type="module" onerror="unreachable()">
  import styleSheet from "../serve-with-content-type.py?fn=css-module/resources/utf-8.css&ct=text/css%3Bcharset=utf-8" with { type: "css"};
  test(() => {
    assert_equals(styleSheet.rules[0].style.content, "\"śćążź\"");
  }, "CSS module should be loaded as utf-8 when charset=utf8 is specified");
</script>
<script type="module" onerror="unreachable()">
  import styleSheet from "../serve-with-content-type.py?fn=css-module/resources/utf-8.css&ct=text/css%3Bcharset=shift-jis" with { type: "css"};
  test(() => {
    assert_equals(styleSheet.rules[0].style.content, "\"śćążź\"");
  }, "CSS module should be loaded as utf-8 when charset=shift-jis is specified");
</script>
<script type="module" onerror="unreachable()">
  import styleSheet from "../serve-with-content-type.py?fn=css-module/resources/utf-8.css&ct=text/css%3Bcharset=windows-1252" with { type: "css"};
  test(() => {
    assert_equals(styleSheet.rules[0].style.content, "\"śćążź\"");
  }, "CSS module should be loaded as utf-8 when charset=windows-1252 is specified");
</script>
<script type="module" onerror="unreachable()">
  import styleSheet from "../serve-with-content-type.py?fn=css-module/resources/utf-8.css&ct=text/css%3Bcharset=utf-7" with { type: "css"};;
  test(() => {
    assert_equals(styleSheet.rules[0].style.content, "\"śćążź\"");
  }, "CSS module should be loaded as utf-8 when charset=utf-7 is specified");
</script>
<script type="module" onerror="unreachable()">
  import styleSheet from "../serve-with-content-type.py?fn=css-module/resources/windows-1250.css&ct=text/css%3Bcharset=windows-1250" with { type: "css"};
  test(() => {
    assert_not_equals(styleSheet.rules[0].style.content, "\"śćążź\"",
                    'Should be decoded as UTF-8');
  }, "CSS module should be loaded as utf-8 even if it is encoded in windows-1250 and served with a windows-1250 charset response header");
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
  "source_name": "html/semantics/scripting-1/the-script-element/css-module/charset.html"
}
```
