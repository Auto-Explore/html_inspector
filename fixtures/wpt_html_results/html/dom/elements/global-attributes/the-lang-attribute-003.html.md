# html/dom/elements/global-attributes/the-lang-attribute-003.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-lang-attribute-003.html",
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
<title>HTTP header</title>
<link rel='author' title='Richard Ishida' href='mailto:ishida@w3.org'>
<link rel='help' href='https://html.spec.whatwg.org/multipage/#the-lang-and-xml:lang-attributes'>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta name='flags' content='http dom'>
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
}, "The browser will recognize a language declared in the HTTP header, when there is no internal language declaration.");
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
        "byte_end": 421,
        "byte_start": 398,
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
  "source_name": "html/dom/elements/global-attributes/the-lang-attribute-003.html"
}
```
