# html/cross-origin-opener-policy/coop-navigated-history-popup.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coop-navigated-history-popup.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Cross-Origin-Opener-Policy: a navigating popup that then goes back in history</title>
<meta name=timeout content=long>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="resources/common.js"></script>
<script>
const title = `Popup navigating to other origin with COOP: same-origin and back in history`;
const channel = title.replace(/ /g,"-");
const opener = false;
const openerDOMAccess = false;
const navigateURL = `${CROSS_ORIGIN.origin}/html/cross-origin-opener-policy/resources/coop-coep.py?coop=same-origin&coep=&navHistory=-1`;

async_test(t => {
  url_test(t, `${SAME_ORIGIN.origin}/html/cross-origin-opener-policy/resources/coop-coep.py?coop=&coep=&navigate=${encodeURIComponent(navigateURL)}&avoidBackAndForth=1&channel=${channel}`, channel, opener, openerDOMAccess);
}, title);
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
  "source_name": "html/cross-origin-opener-policy/coop-navigated-history-popup.https.html"
}
```
