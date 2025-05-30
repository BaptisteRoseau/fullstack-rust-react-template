import { Helmet, HelmetData } from 'react-helmet-async';

interface HeadMetadataProps {
  title?: string;
  description?: string;
};

const helmetData = new HelmetData({});

/**
 * Sets title and description in the page <head> HTML.
 *  
 * @param param0 
 * @returns 
 */
function HeadMetadata({ title = '', description = '' }: HeadMetadataProps = {}) {
  return (
    <Helmet
      helmetData={helmetData}
      title={title ? `${title} | MyCompany` : undefined}
      defaultTitle="MyCompany"
    >
      <meta name="description" content={description} />
    </Helmet>
  );
};

export default HeadMetadata;