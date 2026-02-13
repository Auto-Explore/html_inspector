# html/browsers/origin/origin-keyed-agent-clusters/insecure-http.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/insecure-http.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent requests origin-keying, child requests origin-keying, child is a subdomain of the parent, but all over insecure HTTP</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  insertIframe,
  testSameAgentCluster,
  testGetter
} from "./resources/helpers.mjs";

promise_setup(async () => {
  await insertIframe("{{hosts[][www]}}", "?1");
});

// All origin-keying requests are ignored, since this is over insecure HTTP.
// So both end up in the site-keyed agent cluster.
testSameAgentCluster([self, 0]);

testGetter(self, false, "parent");
testGetter(0, false, "child");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/insecure-http.sub.html"
}
```
