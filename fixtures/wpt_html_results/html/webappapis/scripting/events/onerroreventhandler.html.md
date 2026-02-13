# html/webappapis/scripting/events/onerroreventhandler.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/onerroreventhandler.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>OnErrorEventHandler + ErrorEvent is treated differently</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
var t = async_test("onerror + ErrorEvent + Window");
var t2 = async_test("onerror + !ErrorEvent + Window");
var t3 = async_test("onerror + Document");
</script>
<iframe src="onerroreventhandler-frame.html"></iframe>
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
  "source_name": "html/webappapis/scripting/events/onerroreventhandler.html"
}
```
