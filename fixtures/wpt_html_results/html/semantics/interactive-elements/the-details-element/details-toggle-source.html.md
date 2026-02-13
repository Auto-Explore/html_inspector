# html/semantics/interactive-elements/the-details-element/details-toggle-source.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/details-toggle-source.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9111">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../../popovers/resources/toggle-event-source-test.js"></script>

<details id=details>
  <summary>summary</summary>
  details
</details>

<details id=detailsWithoutSummary>
  details without summary
</details>

<script>
const details = document.getElementById('details');
const detailsWithoutSummary = document.getElementById('detailsWithoutSummary');
const summary = details.querySelector('summary');

async function click(element) {
  // Click halfway up the element to click the activatable summary instead of
  // the details.
  const height = element.getBoundingClientRect().height;
  return (new test_driver.Actions()
    .pointerMove(0, -height / 2, {origin: element})
    .pointerDown()
    .pointerUp())
    .send();
}

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <details> elements: details.open.',
  target: details,
  openFunc: async () => details.open = true,
  closeFunc: async () => details.open = false,
  openSource: null,
  closeSource: null,
  skipBeforetoggle: true
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <details> elements: click summary.',
  target: details,
  openFunc: async () => summary.click(),
  closeFunc: async () => summary.click(),
  openSource: null,
  closeSource: null,
  skipBeforetoggle: true
});

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <details> elements: click details.',
  target: detailsWithoutSummary,
  openFunc: async () => await click(detailsWithoutSummary),
  closeFunc: async () => await click(detailsWithoutSummary),
  openSource: null,
  closeSource: null,
  skipBeforetoggle: true
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 623,
        "byte_start": 613,
        "col": 1,
        "line": 18
      }
    },
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
  "source_name": "html/semantics/interactive-elements/the-details-element/details-toggle-source.html"
}
```
