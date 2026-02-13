# html/semantics/menu/tentative/menuitem-anchor-positioning-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/menuitem-anchor-positioning-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=stylesheet href=resources/menu-elements-styles.css>
<style>
  /* Only buttons can trigger popovers, so style buttons like menuitems, and
   * reset some of their default styles. */
  button {
    box-sizing: content-box;
    font-size: unset;
    font-family: unset;
    text-align: unset;
    background: unset;
    border: 0;
  }
</style>

<div class=menubar>
  <div class=menuitem>Nothing</div>
  <button class=menuitem id=trigger popovertarget=top-menulist>Trigger</button>
</div>

<div popover class="menulist menulist-with-menubar-anchor" id=top-menulist>
  <button class=menuitem id=trigger2 popovertarget=sub-menulist>Trigger 2</button>
</div>

<div popover class="menulist menulist-with-menulist-anchor" id=sub-menulist>
  <div class=menuitem>Most nested item</div>
</div>

<script>
  trigger.click();
  trigger2.click();
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
  "source_name": "html/semantics/menu/tentative/menuitem-anchor-positioning-ref.html"
}
```
