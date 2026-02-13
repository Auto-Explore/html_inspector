# html/browsers/browsing-the-web/history-traversal/pagereveal/resources/iframe.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pagereveal/resources/iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!-- Logs rAF, pageshow, and pagereveal events. Load with '?restore' to ignore
     events prior to pageshow.persisted -->
<script>
// If the test is checking behavior after a BFCache restore, only record once
// pageshow.persisted has been seen.
const params = new URLSearchParams(window.location.search);
let should_record = !params.has('restore');

let event_log = [];

addEventListener('message', async e => {
  if (e.data === 'getEventLog') {
    // Ensure at least one animation frame is produced to ensure
    // pagereveal must have fired.
    await new Promise(requestAnimationFrame);
    e.source.postMessage(event_log, '*');
  }
});

addEventListener('load', () => {
  window.parent.postMessage('loaded', '*');
})

function recordRafs() {
  requestAnimationFrame( () => {
    if (should_record)
      event_log.push('rAF');

    recordRafs();
  });
}

recordRafs();

addEventListener('pageshow', (e) => {
  if (e.persisted)
    should_record = true;

  if (should_record)
    event_log.push('pageshow' + (e.persisted ? '.persisted' : ''));
});

addEventListener('pagereveal', () => {
  if (should_record)
    event_log.push('pagereveal');
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/pagereveal/resources/iframe.html"
}
```
