# html/dom/elements/global-attributes/the-translate-attribute-010.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-translate-attribute-010.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html  lang="en" >
<head>
<meta charset="utf-8"/>
<title>translate inherits no</title>
<link rel='author' title='Richard Ishida' href='mailto:ishida@w3.org'>
<link rel='help' href='https://html.spec.whatwg.org/multipage/#the-translate-attribute'>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style type='text/css'>
</style>
</head>
<body>



<div class="test"><div id="box" translate="no">&nbsp; <span id="spantest">&nbsp;</span></div></div>


<script>
test(function() {
assert_false(document.getElementById('spantest').translate);
}, "If the translate attribute is set to no, javascript will detect the translation mode of elements inside that element with no translate flag as no-translate.");
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
        "byte_end": 392,
        "byte_start": 369,
        "col": 1,
        "line": 10
      }
    }
  ],
  "source_name": "html/dom/elements/global-attributes/the-translate-attribute-010.html"
}
```
