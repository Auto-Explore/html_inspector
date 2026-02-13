# html/cross-origin-opener-policy/tentative/noopener/coop-noopener-allow-popups-restrict-properties.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/tentative/noopener/coop-noopener-allow-popups-restrict-properties.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="timeout" content="long">
<title>
    Cross-Origin-Opener-Policy: noopener-allow-popups means that the opener
    has no access to the openee. This test verfies it for restrict-properties
    COOP values.
</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="../../resources/common.js"></script>
<script src="../../resources/noopener-helper.js"></script>
<script>


test_noopener_opening_popup("noopener-allow-popups",
                            "restrict-properties",
                            SAME_ORIGIN,
                            /*opener_expectation=*/true);
test_noopener_opening_popup("restrict-properties",
                            "noopener-allow-popups",
                            SAME_ORIGIN,
                            /*opener_expectation=*/false);
 </script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/cross-origin-opener-policy/tentative/noopener/coop-noopener-allow-popups-restrict-properties.https.html"
}
```
