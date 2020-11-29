import Image from 'next/image';
import Link from 'next/link';

export default function Header() {
    return (
        <nav>
            <div className="nav-menu">
                <Link href="/">
                    <a>Home</a>
                </Link>
                <Link href="/art">
                    <a>3D</a>
                </Link>
                <Link href="/apps">
                    <a>Apps</a>
                </Link>
                <Link href="/mods">
                    <a>Mods</a>
                </Link>
            </div>
            <div style={{ textAlign: 'center' }}>
                <Link href="/">
                    <a>
                        <Image src="/icon.png" alt="FabianLars' Logo" width={100} height={100} />
                    </a>
                </Link>
            </div>
            <div className="nav-outbound" style={{ textAlign: 'right' }}>
                <a href="https://github.com/FabianLars">
                    <Image src="/img/GitHub-Mark-Light-32px.png" alt="GitHub Logo" width={32} height={32} />
                </a>
            </div>
        </nav>
    );
}
