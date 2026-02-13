# html/syntax/parsing-html-fragments/the-input-byte-stream-037.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing-html-fragments/the-input-byte-stream-037.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
﻿<!DOCTYPE html>
<html  lang="en" >
<head>
 <meta http-equiv="content-type" content="text/html; charset=iso-8859-15"> <title>UTF-8 BOM vs meta content</title>
<link rel='author' title='Richard Ishida' href='mailto:ishida@w3.org'>
<link rel='help' href='https://html.spec.whatwg.org/multipage/#the-input-byte-stream'>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta name='flags' content='http'>
<style type='text/css'>
.test div { width: 50px; }</style>
<link rel="stylesheet" type="text/css" href="support/encodingtests-utf8.css">
</head>
<body>



<div class='test'><div id='box' class='ýäè'>&#xA0;</div></div>


<!--Notes:

The page contains an encoding declaration in a meta content attribute that attempts to set the character encoding to ISO 8859-15, but the file starts with a UTF-8 signature.

The test contains a div with a class name that contains the following sequence of bytes: 0xC3 0xBD 0xC3 0xA4 0xC3 0xA8. These represent different sequences of characters in ISO 8859-15, ISO 8859-1 and UTF-8. The external, UTF-8-encoded stylesheet contains a selector <code>.test div.&#x00FD;&#x00E4;&#x00E8;</code>. This matches the sequence of bytes above when they are interpreted as UTF-8. If the class name matches the selector then the test will pass.

-->
<script>
test(function() {
assert_equals(document.getElementById('box').offsetWidth, 100);
}, "A page with a UTF-8 BOM will be recognized as UTF-8 even if the meta content attribute declares a different encoding.");
</script>

<div id='log'></div>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 483,
        "byte_start": 460,
        "col": 1,
        "line": 10
      }
    }
  ],
  "source_name": "html/syntax/parsing-html-fragments/the-input-byte-stream-037.html"
}
```
