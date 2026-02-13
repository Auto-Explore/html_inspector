# html/browsers/the-window-object/open-close/self-close.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/open-close/self-close.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<p>self-close.html?navs=n&channel=name will navigate n times, then close and notify the channel.</p>

<script>
window.onload = () => setTimeout(() => {
  const urlParams = new URLSearchParams(window.location.search);
  let n = parseInt(urlParams.get('navs')) || 0;

  const channel = new BroadcastChannel(urlParams.get('channel'));

  channel.postMessage({ name: 'load', href: window.location.href });

  if (n > 0) {
    urlParams.set('navs', n-1);
    window.location.href = `${window.location.pathname}?${urlParams.toString()}#${n}`;
  } else {
    window.onbeforeunload = () => {
        channel.postMessage({ name: 'beforeunload', history: history.length, closed: true });
    }
    window.close();
    if (!window.closed) {
        channel.postMessage({ name: 'close failed', history: history.length, closed: false });
    }
  }
}, 0);
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
  "source_name": "html/browsers/the-window-object/open-close/self-close.html"
}
```
