# html/cross-origin-opener-policy/historical/popup-same-site-with-cross-origin.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/historical/popup-same-site-with-cross-origin.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<meta name=timeout content=long>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="../resources/common.js"></script>
<script src="../resources/popup-test.js"></script>
<script>

[
  {
    "title": "popup with empty coop",
    "coop": "",
    "opener": "preserved"
  },
  {
    "title": "popup with coop unsafe-none",
    "coop": "unsafe-none",
    "opener": "preserved"
  },
  {
    "title": "popup with jibberish coop",
    "coop": "jibberish",
    "opener": "preserved"
  },
  {
    "title": "popup with obsolete coop",
    "coop": "same-site",
    "opener": "preserved"
  },
  {
    "title": "popup with obsolete coop 2",
    "coop": "same-site unsafe-allow-outgoing",
    "opener": "preserved"
  },
  {
    "title": "popup with obsolete coop 3",
    "coop": "same-origin unsafe-allow-outgoing",
    "opener": "preserved"
  },
  {
    "title": "popup with coop same-origin",
    "coop": "same-origin",
    "opener": "severed"
  },
  {
    "title": "popup with coop same-origin-allow-popups",
    "coop": "same-origin-allow-popups",
    "opener": "severed"
  }
].forEach(variant => {
  popup_test(`Cross-origin ${variant.title}`, CROSS_ORIGIN, { coop: variant.coop }, variant.opener);
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
  "source_name": "html/cross-origin-opener-policy/historical/popup-same-site-with-cross-origin.https.html"
}
```
