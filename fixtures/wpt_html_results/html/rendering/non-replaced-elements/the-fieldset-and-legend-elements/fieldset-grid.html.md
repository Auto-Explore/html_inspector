# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-grid.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-grid.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset and CSS Grid</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
#test, #ref, #test-inline, #ref-inline {
  display: grid;
  grid-template-columns: auto 50px auto;
  grid-template-rows: auto 50px auto;
  margin: 0;
  padding: 0;
  border: none
}
#test-inline, #ref-inline { display: inline-grid }
legend {
  float: left; /* Makes it not the "rendered legend" */
  padding: 0;
}
</style>
<fieldset id=test>
 <legend>1</legend>
 <div>2</div>
 <div>3</div>
 <div>4</div>
 <div>5</div>
 <div>6</div>
 <div>7</div>
 <div>8</div>
 <div>9</div>
</fieldset>
<hr>
<div id=ref>
 <div>1</div>
 <div>2</div>
 <div>3</div>
 <div>4</div>
 <div>5</div>
 <div>6</div>
 <div>7</div>
 <div>8</div>
 <div>9</div>
</div>
<hr>
<fieldset id=test-inline>
 <legend>1</legend>
 <div>2</div>
 <div>3</div>
 <div>4</div>
 <div>5</div>
 <div>6</div>
 <div>7</div>
 <div>8</div>
 <div>9</div>
</fieldset>
<div id=ref-inline>
 <div>1</div>
 <div>2</div>
 <div>3</div>
 <div>4</div>
 <div>5</div>
 <div>6</div>
 <div>7</div>
 <div>8</div>
 <div>9</div>
</div>
<script>
  test(() => {
    const testElm = document.getElementById('test');
    const refElm = document.getElementById('ref');
    assert_equals(getComputedStyle(testElm).height,
                  getComputedStyle(refElm).height, 'height');
    assert_equals(testElm.querySelector('legend').offsetTop,
                  testElm.querySelector('div').offsetTop, 'offsetTop')
  }, "Grid");

  test(() => {
    const testElm = document.getElementById('test-inline');
    const refElm = document.getElementById('ref-inline');
    assert_equals(getComputedStyle(testElm).height,
                  getComputedStyle(refElm).height, 'height');
    assert_equals(testElm.querySelector('legend').offsetTop,
                  testElm.querySelector('div').offsetTop, 'offsetTop')

  }, "Inline grid");
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-grid.html"
}
```
