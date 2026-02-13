# html/cross-origin-opener-policy/iframe-popup-same-origin-allow-popups-to-same-origin.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/iframe-popup-same-origin-allow-popups-to-same-origin.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<meta name=timeout content=long>
<meta name="variant" content="?1-2">
<meta name="variant" content="?3-4">
<meta name="variant" content="?5-6">
<meta name="variant" content="?7-8">
<meta name="variant" content="?9-last">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/dispatcher/dispatcher.js></script>
<script src=/common/get-host-info.sub.js></script>
<script src=/common/subset-tests.js></script>
<script src=/common/utils.js></script>
<script src="resources/common.js"></script>
<script src="resources/iframe-test.js"></script>

<body>
<script>
[
  {
    "title": "same origin iframe, same origin popup",
    "iframeOrigin": SAME_ORIGIN,
    "popupOrigin": SAME_ORIGIN,
    "opener": "severed"
  },
  {
    "title": "same site iframe, same origin popup",
    "iframeOrigin": SAME_SITE,
    "popupOrigin": SAME_ORIGIN,
    "opener": "severed"
  },
  {
    "title": "cross origin iframe, same origin popup",
    "iframeOrigin": CROSS_ORIGIN,
    "popupOrigin": SAME_ORIGIN,
    "opener": "severed"
  },
  {
    "title": "same origin iframe, same site popup",
    "iframeOrigin": SAME_ORIGIN,
    "popupOrigin": SAME_SITE,
    "opener": "severed"
  },
  {
    "title": "same site iframe, same site popup",
    "iframeOrigin": SAME_SITE,
    "popupOrigin": SAME_SITE,
    "opener": "severed"
  },
  {
    "title": "cross origin iframe, same site popup",
    "iframeOrigin": CROSS_ORIGIN,
    "popupOrigin": SAME_SITE,
    "opener": "severed"
  },
  {
    "title": "same origin iframe, cross origin popup",
    "iframeOrigin": SAME_ORIGIN,
    "popupOrigin": CROSS_ORIGIN,
    "opener": "severed"
  },
  {
    "title": "same site iframe, cross origin popup",
    "iframeOrigin": SAME_SITE,
    "popupOrigin": CROSS_ORIGIN,
    "opener": "severed"
  },
  {
    "title": "cross origin iframe, cross origin popup",
    "iframeOrigin": CROSS_ORIGIN,
    "popupOrigin": CROSS_ORIGIN,
    "opener": "severed"
  }
].forEach(variant => {
  subsetTest(
    iframe_test,
    `COOP: same-origin-allow-popups to popup COOP: same-origin via an iframe, with ${variant.title}`,
    variant.iframeOrigin,
    variant.popupOrigin,
    { coop: 'same-origin' },
    variant.opener);
});
</script>
</body>

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
  "source_name": "html/cross-origin-opener-policy/iframe-popup-same-origin-allow-popups-to-same-origin.https.html"
}
```
