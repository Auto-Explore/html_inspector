# html/syntax/parsing/template/additions-to-the-in-frameset-insertion-mode/end-tag-frameset.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-frameset-insertion-mode/end-tag-frameset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: additions to 'in frameset' insertion mode</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="If parser is in 'in frameset' insertion mode then a start tag or an end tag whose name is 'template' is a parsing error">
<link rel="help" href="https://www.w3.org/TR/2015/WD-html51-20151008/syntax.html#parsing-main-inframeset">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

testInIFrame('/html/semantics/scripting-1/the-template-element/resources/frameset-end-tag.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var frameset = doc.querySelector('frameset');
    assert_equals(frameset.children.length, 0, 'Wrong number of frameset children elements');

}, '<template> tag should be ignored in "in frameset" insertion mode');

</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 182,
        "byte_start": 103,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 182,
        "byte_start": 103,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 664,
        "byte_start": 633,
        "col": 1,
        "line": 14
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-frameset-insertion-mode/end-tag-frameset.html"
}
```
