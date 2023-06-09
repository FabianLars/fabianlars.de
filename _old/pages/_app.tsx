import { AppProps } from 'next/app';
import Head from 'next/head';
import '../styles/globals.scss';

function MyApp({ Component, pageProps }: AppProps) {
    return (
        <>
            <Head>
                <meta name="description" content="Fabian-Lars' personal Website" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="theme-color" content="#051622" />
            </Head>
            <Component {...pageProps} />
        </>
    );
}

export default MyApp;
