# html/semantics/interactive-elements/the-details-element/details-toggle-source-commandfor.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/details-toggle-source-commandfor.tentative.html",
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

<button id=commandsource commandfor=details command=toggle>command source</button>
<details id=details>
  <summary>summary</summary>
  details
</details>

<script>
const details = document.getElementById('details');
const commandsource = document.getElementById('commandsource');

createToggleEventSourceTest({
  description: 'ToggleEvent.source on <details> elements: command invokers.',
  target: details,
  openFunc: async () => commandsource.click(),
  closeFunc: async () => commandsource.click(),
  openSource: commandsource,
  closeSource: commandsource,
  skipBeforetoggle: true
});
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
  "source_name": "html/semantics/interactive-elements/the-details-element/details-toggle-source-commandfor.tentative.html"
}
```
