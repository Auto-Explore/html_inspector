# html/cross-origin-opener-policy/coep-redirect.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coep-redirect.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Cross-Origin-Opener-Policy and Cross-Origin-Embedder-Policy: redirects</title>
<meta name=timeout content=long>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="resources/common.js"></script>
<script>
const coopCOEPPath = new URL("resources/coop-coep.py", window.location).pathname;

[
  {
    "title": "coop/coep to coop/coep",
    "redirectCOOP": "same-origin",
    "redirectCOEP": "require-corp",
    "coop": "same-origin",
    "coep": "require-corp",
    "opener": true
  },
  {
    "title": "coop/coep to no coop/coep",
    "redirectCOOP": "same-origin",
    "redirectCOEP": "require-corp",
    "coop": "",
    "coep": "require-corp",
    "opener": false
  },
  {
    "title": "no coop/no coep to coop/coep",
    "redirectCOOP": "",
    "redirectCOEP": "",
    "coop": "same-origin",
    "coep": "require-corp",
    "opener": false
  },
  {
    "title": "coop/no coep to coop/coep",
    "redirectCOOP": "same-origin",
    "redirectCOEP": "",
    "coop": "same-origin",
    "coep": "require-corp",
    "opener": false
  },
  {
    "title": "coop unsafe-none/coep to coop/coep",
    "redirectCOOP": "unsafe-none",
    "redirectCOEP": "require-corp",
    "coop": "same-origin",
    "coep": "require-corp",
    "opener": false
  },
  {
    "title": "coop unsafe-none/coep to coop unsafe-inherit/coep",
    "redirectCOOP": "unsafe-none",
    "redirectCOEP": "require-corp",
    "coop": "unsafe-none",
    "coep": "require-corp",
    "opener": false
  }
].forEach(variant => {
  const title = `Redirect from ${variant.title}`;
  async_test(t => {
    const channel = title.replace(/ /g,"-");
    const redirectLocation = `${SAME_ORIGIN.origin}${coopCOEPPath}?coop=${variant.coop}&coep=${variant.coep}&channel=${channel}`;
    url_test(t, `${SAME_ORIGIN.origin}${coopCOEPPath}?coop=${variant.redirectCOOP}&coep=${variant.redirectCOEP}&redirect=${encodeURIComponent(redirectLocation)}`, channel, variant.opener);
  }, title);
});
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
  "source_name": "html/cross-origin-opener-policy/coep-redirect.https.html"
}
```
