# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_window_open_download_block_downloads.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_window_open_download_block_downloads.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Downloads triggered by window.open from sandbox are blocked.</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-iframe-sandbox">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-iframe-element">
<script src="/common/utils.js"></script>
<script src="/resources/testharness.js"></script>
<script src='/resources/testharnessreport.js'></script>
<script src="support/iframe_sandbox_download_helper.js"></script>
<body>
<script>
"use strict";

['', '"_blank"', '"_blank", "noopener"'].forEach(options =>
  async_test(t => {
    const download_token = token();
    let iframe = document.createElement("iframe");
    const download_link = `support/download_stash.py?token=${download_token}` +
          `&finish-delay=${StreamDownloadFinishDelay()}`;
    iframe.srcdoc = `<script>window.open("${download_link}", ${options})</scr` +
      `ipt>`;
    iframe.sandbox = "allow-same-origin allow-popups allow-scripts";
    AssertDownloadFailure(t, download_token, StreamDownloadFinishDelay() +
                          DownloadVerifyDelay());
    document.body.appendChild(iframe);
  }, `window.open(download, ${options}) triggering download in ` +
             'sandbox is blocked.'));
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_window_open_download_block_downloads.tentative.html"
}
```
