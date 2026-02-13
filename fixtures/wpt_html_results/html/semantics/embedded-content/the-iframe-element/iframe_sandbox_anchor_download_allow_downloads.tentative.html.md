# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_anchor_download_allow_downloads.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_anchor_download_allow_downloads.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>&lt;a download&gt; triggered download in sandbox is allowed by allow-downloads.</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-iframe-sandbox">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-iframe-element">
<script src="/common/utils.js"></script>
<script src="/resources/testharness.js"></script>
<script src='/resources/testharnessreport.js'></script>
<script src="support/iframe_sandbox_download_helper.js"></script>
<body>
<script>
"use strict";

const attributes_list = [
  '',
  'target="_blank"',
  'target="_blank" rel="noopener"',
];

const download_flags = [false, true];

attributes_list.forEach(attributes =>
  download_flags.forEach(download_flag =>
    async_test(t => {
      const download_token = token();
      let iframe = document.createElement("iframe");
      iframe.srcdoc = `<a ${attributes}>Download</a>`;
      iframe.sandbox = "allow-same-origin allow-popups allow-downloads";
      iframe.addEventListener('load', t.step_func(function () {
        if (attributes !== '' || download_flag) {
          // Specifiying `download` or a `target` should not trigger a
          // navigation in this iframe.
          iframe.contentWindow.addEventListener(
            "unload", t.unreached_func("Unexpected navigation."));
        }
        let anchor = iframe.contentDocument.getElementsByTagName('a')[0];
        anchor.href = "support/download_stash.py?token=" + download_token;
        if (download_flag) anchor.download = null;
        anchor.click();
        AssertDownloadSuccess(t, download_token, DownloadVerifyDelay());
      }), { once: true });

      document.body.appendChild(iframe);
    }, `<a ${attributes} ${download_flag ? "download" : ""}> triggered ` +
               `download in sandbox is allowed by allow-downloads.`)));

</script>
</body>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_anchor_download_allow_downloads.tentative.html"
}
```
