# html/dom/elements/global-attributes/the-lang-attribute-004.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-lang-attribute-004.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html   >
<head>
<meta charset="utf-8"/>
 <meta http-equiv="Content-Language" content="ko" >
<title>pragma-set default</title>
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
}, "The browser will recognize a language declared in a meta element in the head using http-equiv='Content-Language' content='..' (with a single language tag value), when there is no other language declaration inside the document.");
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
        "byte_end": 108,
        "byte_start": 58,
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
        "byte_end": 475,
        "byte_start": 452,
        "col": 1,
        "line": 12
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
  "source_name": "html/dom/elements/global-attributes/the-lang-attribute-004.html"
}
```
