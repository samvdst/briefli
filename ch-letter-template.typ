#let ch-letter(
  sender: (
    name: none,
    address: none,
    extra: none,
  ),
  recipient: none,
  location: none,
  date: none,
  subject: none,
  footer: none,
  font: "Arial",
  address-position: "left",
  body,
) = {
  // Swiss C5 envelope address window measurements (A4 folded once)
  // Left window:  22mm from left edge, 45mm from top (sender), 60mm from top (recipient)
  // Right window: 118mm from left edge (same vertical positions)
  // Address area: 85.5mm × 45mm (fits 5-6 lines)
  let address-left-pos = 22mm
  let address-right-pos = 118mm
  let address-top = 60mm
  let address-width = 85.5mm
  let sender-top = 45mm

  // Margins based on address position
  let margin-left = if address-position == "right" { 20mm } else { address-left-pos }
  let margin-right = if address-position == "right" { 210mm - address-right-pos - address-width } else { 20mm }
  let margin-top = 20mm
  let margin-bottom = 20mm

  // Metadata
  if sender.name != none {
    set document(title: subject, author: sender.name)
  } else {
    set document(title: subject)
  }

  // Page setup
  set page(
    paper: "a4",
    margin: (
      left: margin-left,
      right: margin-right,
      top: margin-top,
      bottom: margin-bottom,
    ),
    footer-descent: 0%,
    footer: {
      if footer != none {
        set text(size: 8pt)
        align(center, footer)
      }
    },
  )

  // Global text style
  set text(
    font: font,
    size: 11pt,
    hyphenate: false,
  )

  // Sender as return address line (small, underlined, above address window)
  if sender.name != none or sender.address != none or sender.at("extra", default: none) != none {
    let sender-x = if address-position == "right" { address-right-pos } else { address-left-pos }
    place(
      top + left,
      dx: sender-x - margin-left,
      dy: sender-top - margin-top,
      box(
        width: address-width,
        {
          set text(size: 8pt)
          let sender-parts = ()
          if sender.name != none {
            sender-parts.push(sender.name)
          }
          if sender.address != none {
            sender-parts.push(sender.address)
          }
          if sender.at("extra", default: none) != none {
            sender-parts.push(sender.extra)
          }
          underline(sender-parts.join(" · "))
        },
      ),
    )
  }

  // Recipient block - absolutely positioned for envelope window
  if recipient != none {
    let recipient-x = if address-position == "right" { address-right-pos } else { address-left-pos }
    place(
      top + left,
      dx: recipient-x - margin-left,
      dy: address-top - margin-top,
      box(
        width: address-width,
        {
          set text(size: 11pt)
          if type(recipient) == str {
            recipient.split(", ").join(linebreak())
          } else {
            recipient
          }
        },
      ),
    )
  }

  // Content starts after the address window area (60mm top + 45mm height = 105mm)
  v(105mm - margin-top)

  // Place + date (left aligned, Swiss standard)
  if location != none or date != none {
    if location != none {
      location
      if date != none { ", " }
    }
    if date != none { date }
    v(8mm)
  }

  // Subject
  if subject != none {
    strong(subject)
    v(8mm)
  }

  // Body
  set par(justify: true)
  body
}
