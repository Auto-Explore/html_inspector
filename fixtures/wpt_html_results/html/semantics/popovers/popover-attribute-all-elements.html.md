# html/semantics/popovers/popover-attribute-all-elements.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-attribute-all-elements.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/popover-utils.js"></script>
<script src="../../resources/common.js"></script>

<body>
<script>
setup({ explicit_done: true });
window.onload = () => {
  // Loop through all HTML elements that render a box by default:
  let elementsThatDontRender = ['area', 'audio','base','br','datalist','dialog','embed','head','link','meta','noscript','optgroup','option','param','rp','script','slot','style','template','title','wbr'];
  const elements = HTML5_ELEMENTS.filter(el => !elementsThatDontRender.includes(el));
  elements.forEach(tag => {
    test((t) => {
      const element = document.createElement(tag);
      element.setAttribute('popover','auto');
      document.body.appendChild(element);
      t.add_cleanup(() => element.remove());
      assertIsFunctionalPopover(element, true);
    }, `A <${tag} popover> element should behave as a popover.`);
    test((t) => {
      const element = document.createElement(tag);
      document.body.appendChild(element);
      t.add_cleanup(() => element.remove());
      assertNotAPopover(element);
    }, `A <${tag}> element should *not* behave as a popover.`);
  });
  elementsThatDontRender.forEach(tag => {
    test((t) => {
      const element = document.createElement(tag);
      element.setAttribute('popover','auto');
      document.body.appendChild(element);
      t.add_cleanup(() => element.remove());
      assertIsFunctionalPopover(element, false);
    }, `A <${tag} popover> element should not be rendered.`);
  });
  done();
};
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
  "source_name": "html/semantics/popovers/popover-attribute-all-elements.html"
}
```
