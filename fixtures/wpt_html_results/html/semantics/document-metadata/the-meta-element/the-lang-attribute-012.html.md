# html/semantics/document-metadata/the-meta-element/the-lang-attribute-012.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/the-lang-attribute-012.html",
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
 <meta http-equiv="Content-Language" content="ko,zh,ja" >
<title>Multiple languages in Content-Language meta element</title>
<link rel='author' title='Richard Ishida' href='mailto:ishida@w3.org'>
<link rel='help' href='https://html.spec.whatwg.org/multipage/#pragma-directives'>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta name='flags' content='dom'>
<style type='text/css'>
    #colonlangcontroltest { color: red; font-weight: bold; width: 400px; }
    #colonlangcontroltest:lang(xx) { display:none; }
.test div { width: 50px; }

#box:lang(ko) { width: 100px; }
#box:lang(zh) { width: 100px; }
#box:lang(ja) { width: 100px; }

    /* styling for debugging related notes */
     .notes span:lang(ko) { background-color: #0000FF; color: white; padding: 0 5px; }
     .notes span:lang(zh) { background-color: #0000FF; color: white; padding: 0 5px; }
     .notes span:lang(ja) { background-color: #0000FF; color: white; padding: 0 5px; }

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
}, "The UA will not recognize a language declaration in the Content-Language meta element when more than one language is declared.");
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
        "byte_end": 114,
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
        "byte_end": 499,
        "byte_start": 476,
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
  "source_name": "html/semantics/document-metadata/the-meta-element/the-lang-attribute-012.html"
}
```
