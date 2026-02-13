# html/dom/elements/global-attributes/data_unicode_attr.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/data_unicode_attr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>HTML Test: dataset attribute</title>
<link rel="author" title="ElegantPig" href="mailto:neil.ep@hotmail.com">
<link rel="author" title="Xiaojun Wu" href="mailto:xiaojunx.a.wu@intel.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id='log'></div>
<div id="d1" data-weapons="laser 2" data-中文属性="中文"></div>
<script>

test(function() {
  var d1 = document.getElementById("d1");
  assert_equals(d1.dataset.weapons, "laser 2");
}, "dataset - SBCS");

test(function() {
  var d1 = document.getElementById("d1");
  assert_equals(d1.dataset.中文属性, "中文");
}, "dataset - UNICODE");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.data_attribute.not_xml_serializable",
      "message": "“data-*” attribute names must be XML 1.0 4th ed. plus Namespaces NCNames.",
      "severity": "Warning",
      "span": {
        "byte_end": 423,
        "byte_start": 360,
        "col": 1,
        "line": 9
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
  "source_name": "html/dom/elements/global-attributes/data_unicode_attr.html"
}
```
