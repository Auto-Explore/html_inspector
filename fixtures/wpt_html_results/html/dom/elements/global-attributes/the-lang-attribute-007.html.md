# html/dom/elements/global-attributes/the-lang-attribute-007.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-lang-attribute-007.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html   lang="ko" >
<head>
<meta charset="utf-8"/>
 <meta http-equiv="Content-Language" content="zh" >
<title>html lang and meta elements</title>
<link rel='author' title='Richard Ishida' href='mailto:ishida@w3.org'>
<link rel='help' href='https://html.spec.whatwg.org/multipage/#the-lang-and-xml:lang-attributes'>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta name='flags' content='dom'>
<style type='text/css'>
    #colonlangcontroltest { color: red; font-weight: bold; width: 400px; }
    #colonlangcontroltest:lang(xx) { display:none; }
.test div { width: 50px; }
#box:lang(ko) { width: 100px; }
</style>
</head>
<body>



<div class="test"><div id="box">&#xA0;</div></div>
<p lang='xx' id='colonlangcontroltest'>This test failed because it relies on :lang for results, but :lang is not supported by this browser.</p>


<!--Notes:

This test uses :lang to detect whether the language has been set. If :lang is not supported, a message will appear and the test will fail.

-->
<script>
test(function() {
assert_equals(document.getElementById('colonlangcontroltest').offsetWidth, 0)
assert_equals(document.getElementById('box').offsetWidth, 100);
}, "If there is a conflict between the language declared using lang in the html element and that in the meta element, the UA will recognize the language declared in the html element.");
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
      "code": "html.meta.http_equiv.content_language.obsolete",
      "message": "Using the “meta” element to specify the document-wide default language is obsolete. Consider specifying the language on the root element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 118,
        "byte_start": 68,
        "col": 2,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 494,
        "byte_start": 471,
        "col": 1,
        "line": 12
      }
    }
  ],
  "source_name": "html/dom/elements/global-attributes/the-lang-attribute-007.html"
}
```
