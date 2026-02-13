# html/syntax/parsing-html-fragments/the-input-byte-stream-007.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing-html-fragments/the-input-byte-stream-007.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html  lang="en" >
<head>
 <meta http-equiv="content-type" content="text/html; charset=iso-8859-15"> <title>meta content attribute</title>
<link rel='author' title='Richard Ishida' href='mailto:ishida@w3.org'>
<link rel='help' href='https://html.spec.whatwg.org/multipage/#the-input-byte-stream'>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta name='flags' content='http'>
<style type='text/css'>
.test div { width: 50px; }</style>
<link rel="stylesheet" type="text/css" href="support/encodingtests-15.css">
</head>
<body>



<div class='test'><div id='box' class='ýäè'>&#xA0;</div></div>


<!--Notes:

The only character encoding declaration for this HTML file is in the content attribute of the meta element, which declares the encoding to be ISO 8859-15.

The test contains a div with a class name that contains the following sequence of bytes: 0xC3 0xBD 0xC3 0xA4 0xC3 0xA8. These represent different sequences of characters in ISO 8859-15, ISO 8859-1 and UTF-8. The external, UTF-8-encoded stylesheet contains a selector <code>.test div.&#x00C3;&#x0153;&#x00C3;&#x20AC;&#x00C3;&#x0161;</code>. This matches the sequence of bytes above when they are interpreted as ISO 8859-15. If the class name matches the selector then the test will pass.

-->
<script>
test(function() {
assert_equals(document.getElementById('box').offsetWidth, 100);
}, "The character encoding of the page can be set by a meta element with http-equiv and content attributes.");
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
        "byte_end": 477,
        "byte_start": 454,
        "col": 1,
        "line": 10
      }
    }
  ],
  "source_name": "html/syntax/parsing-html-fragments/the-input-byte-stream-007.html"
}
```
