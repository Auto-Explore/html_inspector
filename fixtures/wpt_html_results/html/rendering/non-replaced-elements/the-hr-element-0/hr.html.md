# html/rendering/non-replaced-elements/the-hr-element-0/hr.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-hr-element-0/hr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>The hr element</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
#ref {
  display: block;
  unicode-bidi: isolate;
  color: gray;
  border-style: inset;
  border-width: 1px;
  margin: 0.5em auto;
  /* TODO: uncomment this when these properties are widely supported
  margin-block-start: 0.5em;
  margin-inline-end: auto;
  margin-block-end: 0.5em;
  margin-inline-start: auto;
  */
  overflow: hidden;
}
</style>

<hr id=test>
<div id=ref></div>

<script>
setup(() => {
  self.testStyle = getComputedStyle(document.getElementById('test'));
  self.refStyle = getComputedStyle(document.getElementById('ref'));
});
['display',
 'unicodeBidi',
 'color',
 'borderTopStyle',
 'borderRightStyle',
 'borderBottomStyle',
 'borderLeftStyle',
 'borderTopWidth',
 'borderRightWidth',
 'borderBottomWidth',
 'borderLeftWidth',
 'marginTop',
 'marginRight',
 'marginBottom',
 'marginLeft',
 'overflow',
 // Extra tests
 'height',
 'box-sizing',
].forEach(prop => {
  test(() => {
    assert_equals(testStyle[prop], refStyle[prop]);
  }, prop);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-hr-element-0/hr.html"
}
```
