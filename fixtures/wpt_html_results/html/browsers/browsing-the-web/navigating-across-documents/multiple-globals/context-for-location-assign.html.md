# html/browsers/browsing-the-web/navigating-across-documents/multiple-globals/context-for-location-assign.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/multiple-globals/context-for-location-assign.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/context-helper.js"></script>
<script>
window.scriptToRun = 'relevantWindow.location.assign("target.html");';

async_test(t => {
  window.addEventListener("message", t.step_func_done(function(e) {
    // Base URL used for parsing a relative URL to `target.html`
    // should be the base URL of the entry settings object in
    // https://html.spec.whatwg.org/C/#dom-location-assign
    assert_equals(
        e.data.location,
        new URL('target.html', entryUrl).href,
        'Base URL should use the entry settings object');

    // `document.referrer` should reflect the source browsing context,
    // which is the incumbent in
    // https://html.spec.whatwg.org/C/#location-object-navigate
    assert_equals(
        e.data.referrer, incumbentUrl,
        'Referrer should use the incumbent settings object');
  }));
}, 'Fetch client and URL resolution for location.assign()');
</script>
<iframe id="entry" src="entry/entry.html"></iframe>
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/multiple-globals/context-for-location-assign.html"
}
```
