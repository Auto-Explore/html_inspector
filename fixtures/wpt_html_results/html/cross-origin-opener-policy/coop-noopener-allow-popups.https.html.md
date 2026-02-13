# html/cross-origin-opener-policy/coop-noopener-allow-popups.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coop-noopener-allow-popups.https.html",
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
    has no access to the openee.
</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="./resources/common.js"></script>
<script src="./resources/noopener-helper.js"></script>
<script>


test_noopener_opening_popup("noopener-allow-popups",
                            "unsafe-none",
                            SAME_ORIGIN,
                            /*opener_expectation=*/true);
test_noopener_opening_popup("noopener-allow-popups",
                            "noopener-allow-popups",
                            SAME_ORIGIN,
                            /*opener_expectation=*/false);
test_noopener_opening_popup("noopener-allow-popups",
                            "same-origin",
                            SAME_ORIGIN,
                            /*opener_expectation=*/false);
test_noopener_opening_popup("noopener-allow-popups",
                            "same-origin-allow-popups",
                            SAME_ORIGIN,
                            /*opener_expectation=*/false);
test_noopener_opening_popup("noopener-allow-popups",
                            "same-origin-allow-popups",
                            CROSS_ORIGIN,
                            /*opener_expectation=*/false);
test_noopener_opening_popup("same-origin-allow-popups",
                            "noopener-allow-popups",
                            SAME_ORIGIN,
                            /*opener_expectation=*/false);
test_noopener_opening_popup("same-origin",
                            "noopener-allow-popups",
                            SAME_ORIGIN,
                            /*opener_expectation=*/false);
test_noopener_opening_popup("unsafe-none",
                            "noopener-allow-popups",
                            SAME_ORIGIN,
                            /*opener_expectation=*/false);
test_noopener_navigating_away("unsafe-none");
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
  "source_name": "html/cross-origin-opener-policy/coop-noopener-allow-popups.https.html"
}
```
