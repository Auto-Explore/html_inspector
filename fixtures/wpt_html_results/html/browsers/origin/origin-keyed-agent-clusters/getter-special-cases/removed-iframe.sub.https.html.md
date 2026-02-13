# html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/removed-iframe.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/removed-iframe.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>window.originAgentCluster for a removed frame</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import { navigateIframe } from "../resources/helpers.mjs";

promise_test(async () => {
  // We cannot use insertIframe because it sets both `document.domain`s. That
  // shouldn't matter, but Chrome has a bug (https://crbug.com/1095145), so
  // let's avoid making the test needlessly fail because of that bug.
  const iframe = document.createElement("iframe");
  const navigatePromise = navigateIframe(iframe, "{{hosts[][]}}", "?1");
  document.body.append(iframe);
  await navigatePromise;

  const frameWindow = iframe.contentWindow;

  assert_equals(frameWindow.originAgentCluster, true, "before");
  iframe.remove();
  assert_equals(frameWindow.originAgentCluster, true, "after");
}, "Removing the iframe does not change originAgentCluster");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/getter-special-cases/removed-iframe.sub.https.html"
}
```
