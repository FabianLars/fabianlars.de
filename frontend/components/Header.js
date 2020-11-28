import Image from 'next/image';
import Link from 'next/link';

export default function Header() {
    return (
        <nav>
            <div className="nav-menu"></div>
            <div style={{ textAlign: 'center' }}>
                <Link href="/">
                    <Image src="/icon.png" alt="FabianLars' Logo" width={100} height={100}></Image>
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
