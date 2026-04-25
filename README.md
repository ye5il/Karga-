# Karga News Reader

Türkçe Terminal Haber Okuyucu - RSS News Reader

## Özellikler

- Türkçe ve Dünya haberleri
- 10 kategori: Haber, Dünya, Spor, Politika, Ekonomi, Teknoloji, Kültür, Sağlık, Bilim, Güvenlik
- Tab/Shift+Tab ile kategori navigasyonu
- Enter ile haber detayı
- Esc ile geri dönüş
- Klavye kısayolları: h/w/s/p/e/t/c/a/b/g/q

## Kurulum

### Arch Linux (AUR)

```bash
yay -S karga-bin
```

### Manuel Kurulum

```bash
# Binary indir
wget https://github.com/ye5il/Karga-/releases/download/v0.1.0/karga

# Çalıştırılabilir yap
chmod +x karga

# Çalıştır
./karga
```

### Kaynak Koddan Derleme

```bash
git clone https://github.com/ye5il/Karga-.git
cd Karga
cargo build --release
./target/release/karga
```

## Bağımlılıklar

- ratatui
- crossterm
- reqwest
- tokio
- rss
- atom_syndication
- serde
- serde_json
- chrono
- notify-rust
- scraper
- regex
- once_cell
- dirs

## Kullanım

```
[h] Haber    [w] Dünya    [s] Spor    [p] Politika
[e] Ekonomi [t] Teknoloji [c] Kültür  [a] Sağlık
[b] Bilim   [g] Güvenlik [q] Çıkış

[Tab] Sonraki kategori
[Enter] Haber detayı
[Esc] Geri dön
```

## Sistem Gereksinimleri

- Linux
- Terminal (xterm, vt100 destekli)
- İnternet bağlantısı (RSS feed'ler için)

## Lisans

MIT License