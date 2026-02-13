# html/semantics/scripting-1/the-script-element/module/charset-01.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/charset-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Root module scripts should always use UTF-8</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="module" src="../serve-with-content-type.py?fn=external-script-utf8.js&ct=text/javascript&dummy=1"></script>
<script type="module">
test(function() {
  assert_equals(window.getSomeString(), "śćążź",
                'Should be decoded as UTF-8');
}, 'UTF-8 module script');
</script>

<script type="module" src="../serve-with-content-type.py?fn=external-script-utf8.js&ct=text/javascript&dummy=2" charset="windows-1250"></script>
<script type="module">
test(function() {
  assert_equals(window.getSomeString(), "śćążź",
                'Should be decoded as UTF-8');
}, 'UTF-8 module script with wrong charset in attribute');
</script>

<script type="module" src="../serve-with-content-type.py?fn=external-script-utf8.js&ct=text/javascript%3Bcharset=windows-1250&dummy=3"></script>
<script type="module">
test(function() {
  assert_equals(window.getSomeString(), "śćążź",
                'Should be decoded as UTF-8');
}, 'UTF-8 module script with wrong charset in Content-Type');
</script>

<script type="module" src="../serve-with-content-type.py?fn=external-script-windows1250.js&ct=text/javascript&dummy=1"></script>
<script type="module">
test(function() {
  assert_not_equals(window.getSomeString(), "śćążź",
                    'Should be decoded as UTF-8');
}, 'Non-UTF-8 module script');
</script>

<script type="module" src="../serve-with-content-type.py?fn=external-script-windows1250.js&ct=text/javascript&dummy=2" charset="windows-1250"></script>
<script type="module">
test(function() {
  assert_not_equals(window.getSomeString(), "śćążź",
                    'Should be decoded as UTF-8');
}, 'Non-UTF-8 module script with charset in attribute');
</script>

<script type="module" src="../serve-with-content-type.py?fn=external-script-windows1250.js&ct=text/javascript%3Bcharset=windows-1250"></script>
<script type="module">
test(function() {
  assert_not_equals(window.getSomeString(), "śćążź",
                    'Should be decoded as UTF-8');
}, 'Non-UTF-8 module script with charset in Content-Type');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.charset.utf8_only",
      "message": "The only allowed value for the “charset” attribute for the “script” element is “utf-8”. (But the attribute is not needed and should be omitted altogether.)",
      "severity": "Warning",
      "span": {
        "byte_end": 641,
        "byte_start": 506,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.script.charset.utf8_only",
      "message": "The only allowed value for the “charset” attribute for the “script” element is “utf-8”. (But the attribute is not needed and should be omitted altogether.)",
      "severity": "Warning",
      "span": {
        "byte_end": 1686,
        "byte_start": 1544,
        "col": 1,
        "line": 38
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/charset-01.html"
}
```
