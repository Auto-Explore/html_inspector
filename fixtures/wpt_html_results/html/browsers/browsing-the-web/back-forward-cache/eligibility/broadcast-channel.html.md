# html/browsers/browsing-the-web/back-forward-cache/eligibility/broadcast-channel.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/back-forward-cache/eligibility/broadcast-channel.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="../resources/helper.sub.js"></script>
<script>
// Check whether the page is BFCached when there are open BroadcastChannels.
// See https://github.com/whatwg/html/issues/7219 for other related scenarios.
runEventTest(
  {funcBeforeNavigation: () => {
      window.bc = new BroadcastChannel('foo');
  }},
  'Eligibility (BroadcastChannel)');

// Same as above, but the BroadcastChannels are closed in the pagehide event.
runEventTest(
  {funcBeforeNavigation: () => {
      window.bc = new BroadcastChannel('foo');
      window.addEventListener('pagehide', () => window.bc.close());
  }},
  'Eligibility (BroadcastChannel closed in the pagehide event)');
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
  "source_name": "html/browsers/browsing-the-web/back-forward-cache/eligibility/broadcast-channel.html"
}
```
