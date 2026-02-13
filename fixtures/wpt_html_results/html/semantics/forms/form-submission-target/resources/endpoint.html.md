# html/semantics/forms/form-submission-target/resources/endpoint.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-target/resources/endpoint.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script>
  const channelName = new URL(location).searchParams.get("channelname"),
        channel = new BroadcastChannel(channelName);
  channel.postMessage({ haveOpener: window.opener !== null,
                        referrer: document.referrer });
  // Because messages are not delivered synchronously and because closing a
  // browsing context prompts the eventual clearing of all task sources, this
  // document should not be closed until the opener document has confirmed
  // receipt.
  channel.onmessage = () => window.close();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 8,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
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
  "source_name": "html/semantics/forms/form-submission-target/resources/endpoint.html"
}
```
