# html/browsers/browsing-the-web/back-forward-cache/eligibility/inflight-fetch-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/back-forward-cache/eligibility/inflight-fetch-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="../resources/helper.sub.js"></script>
<script src="../resources/inflight-fetch-helper.js"></script>
<script>
// Check whether the page is BFCached when there are in-flight network requests
// at the time of navigation.

// Successful fetch completion with header received when in BFCache or after
// BFCache.
runTest(sameOriginUrl + '?delayBeforeHeader=2000', false, true,
  'Header and body received when in BFCache');
runTest(sameOriginUrl + '?delayBeforeHeader=2000&delayBeforeBody=1500',
  false, true,
  'Header received when in BFCache and body received after BFCache');
runTest(sameOriginUrl + '?delayBeforeHeader=3500', false, true,
  'Header and body received after BFCache');
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
  "source_name": "html/browsers/browsing-the-web/back-forward-cache/eligibility/inflight-fetch-2.html"
}
```
