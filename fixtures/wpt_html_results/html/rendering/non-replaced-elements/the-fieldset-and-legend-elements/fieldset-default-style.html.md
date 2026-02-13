# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-default-style.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-default-style.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset default style</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
#ref {
  display: block;
  margin-left: 2px;
  margin-right: 2px;
  /* TODO replace above declarations with these when they are widely supported.
  margin-inline-start: 2px;
  margin-inline-end: 2px;
  */
  border: groove 2px;
  padding: 0.35em 0.75em 0.625em 0.75em;
  /* TODO replace above declarations with these when they are widely supported.
  padding-block-start: 0.35em;
  padding-inline-end: 0.75em;
  padding-block-end: 0.625em;
  padding-inline-start: 0.75em;
  */
  min-width: min-content;
  /* TODO change the above to min-inline-size when it's widely supported. */
}
</style>
<fieldset id=test></fieldset>
<div id=ref></div>
<script>
  const testElm = document.querySelector('#test');
  const refElm = document.querySelector('#ref');
  const props = ['display',
                 'margin-top',
                 'margin-right',
                 'margin-bottom',
                 'margin-left',
                 'border-top-style',
                 'border-right-style',
                 'border-bottom-style',
                 'border-left-style',
                 'border-top-width',
                 'border-right-width',
                 'border-bottom-width',
                 'border-left-width',
                 'padding-top',
                 'padding-right',
                 'padding-bottom',
                 'padding-left',
                 'min-width',
                 ];
  const testStyle = getComputedStyle(testElm);
  const refStyle = getComputedStyle(refElm);
  props.forEach(prop => {
    test(() => {
      assert_equals(testStyle[prop], refStyle[prop]);
    }, `${prop}`);
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-default-style.html"
}
```
