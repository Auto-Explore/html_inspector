# html/dom/elements/global-attributes/the-lang-attribute-002.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-lang-attribute-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html   xml:lang="ko" >
<head>
<meta charset="utf-8"/>
<title>xml:lang attribute in html tag</title>
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
assert_equals(document.getElementById('box').offsetWidth, 50);
}, "The browser will NOT recognize a language declared in an xml:lang attribute on the html tag.");
</script>

<div id='log'></div>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.xml_lang.requires_lang",
      "message": "When the attribute “xml:lang” in no namespace is specified, the element must also have the attribute “lang” present with the same value.",
      "severity": "Error",
      "span": {
        "byte_end": 39,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 449,
        "byte_start": 426,
        "col": 1,
        "line": 11
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
  "source_name": "html/dom/elements/global-attributes/the-lang-attribute-002.html"
}
```
