# html/rendering/widgets/button-layout/shrink-wrap.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/shrink-wrap.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>shrink-wrap button</title>
<link rel=help href=https://html.spec.whatwg.org/multipage/rendering.html#button-layout>
<link rel=help href=https://drafts.csswg.org/css2/visudet.html#shrink-to-fit-float>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<!--
 minimum preferred width 100px
 preferred width         250px
 available width         vary
-->
<style>
div.available-width { border: thin dotted red; margin: 1em 0; padding: 1em 0 }
button { border: none; margin: 0; padding: 0; background: papayawhip; }
button > span.minimum-preferred-width { width: 100px }
button > span { width: 50px; display: inline-block; outline: thin dotted blue }
</style>
<div class="available-width" style="width: 50px">
 <button data-expected="100"><span class="minimum-preferred-width">x</span><span>x</span><span>x</span><span>x</span></button>
</div>

<div class="available-width" style="width: 200px">
 <button data-expected="200"><span class="minimum-preferred-width">x</span><span>x</span><span>x</span><span>x</span></button>
</div>

<div class="available-width" style="width: 300px">
 <button data-expected="250"><span class="minimum-preferred-width">x</span><span>x</span><span>x</span><span>x</span></button>
</div>

<script>
const styles = ['display: block', 'display: inline', 'display: inline-block',
                'display: list-item', 'display: table', 'display: table-row',
                'display: table-cell', 'float: left'];
for (const style of styles) {
  for (const button of [].slice.call(document.querySelectorAll('button'))) {
    test(() => {
      button.setAttribute('style', style);
      assert_equals(button.clientWidth, parseInt(button.dataset.expected, 10));
    }, `${style} - available ${button.parentNode.getAttribute('style')}`);
  }
}
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
  "source_name": "html/rendering/widgets/button-layout/shrink-wrap.html"
}
```
