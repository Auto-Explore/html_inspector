# html/semantics/forms/the-select-element/customizable-select/picker-and-slotted.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/picker-and-slotted.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<link rel=match href="picker-and-slotted-ref.html">
<link rel="author" title="L. David Baron" href="https://dbaron.org/">
<link rel="author" title="Google" href="http://www.google.com/">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<div id="host">
  <select>
    <option>one</option>
    <option>two</option>
  </select>
</div>

<script>

let host = document.getElementById("host");
let shadow = host.attachShadow({mode: "open"});
shadow.innerHTML = `
  <style>
    #myslot::slotted(select), #myslot::slotted(select)::picker(select) {
      appearance: base-select;
    }
    #myslot::slotted(select)::picker(select) {
      border: thick solid blue;
      background: lime;
      color: maroon;
    }
  </style>
  <slot id="myslot"></slot>
`;


(async () => {
  await test_driver.bless();
  document.querySelector('select').showPicker();
  document.documentElement.classList.remove('reftest-wait');
})();

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-select-element/customizable-select/picker-and-slotted.html"
}
```
